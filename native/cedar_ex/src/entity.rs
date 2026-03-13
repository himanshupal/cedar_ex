use cedar_policy::{Entities, Entity, EntityUid};
use rustler::{Error, NifResult, NifStruct, NifTaggedEnum, ResourceArc, nif};

use crate::{
    atoms,
    common::{ExEntityUid, ExFormat, ExRecordItem, ExRecordItems, RecordItems},
    error::ExError,
    schema::parse_schema,
    state::State,
};

#[derive(NifTaggedEnum, Debug)]
enum ExEntityFormat<'a> {
    List(Vec<ExEntity>),
    Json(&'a str),
}

#[derive(NifStruct, Debug)]
#[module = "CedarPolicy.Entity"]
pub(crate) struct ExEntity {
    pub(crate) id: ExEntityUid,
    pub(crate) tags: Vec<ExRecordItem>,
    pub(crate) attrs: Vec<ExRecordItem>,
    pub(crate) parents: Vec<ExEntityUid>,
}

#[nif]
pub(crate) fn add_entities(
    ctx: ResourceArc<State>,
    entities: ExEntityFormat,
    schema: Option<ExFormat>,
) -> NifResult<ResourceArc<State>> {
    let s = parse_schema(schema)?;

    let e: Vec<Entity> = match entities {
        ExEntityFormat::List(value) => value
            .into_iter()
            .map(|entity| {
                let id: NifResult<EntityUid> = entity.id.into();
                let attrs: NifResult<RecordItems> = ExRecordItems(entity.attrs).into();
                let tags: NifResult<RecordItems> = ExRecordItems(entity.tags).into();

                let parents = entity
                    .parents
                    .into_iter()
                    .map(|v| v.into())
                    .collect::<NifResult<Vec<EntityUid>>>()?;

                Entity::new_with_tags(id?, attrs?, parents, tags?).map_err(|e| {
                    Error::Term(Box::new(ExError {
                        source: atoms::entity(),
                        reason: e.inner().to_string(),
                    }))
                })
            })
            .collect(),
        ExEntityFormat::Json(value) => Ok(Entities::from_json_str(value, s.as_ref())
            .map_err(|e| {
                Error::Term(Box::new(ExError {
                    source: atoms::entity(),
                    reason: e.to_string(),
                }))
            })?
            .into_iter()
            .collect()),
    }?;

    {
        let mut entities = ctx.entities.write().unwrap();

        let current = entities.clone();
        // FIXME: Better error handling

        *entities = current.add_entities(e, s.as_ref()).map_err(|e| {
            Error::Term(Box::new(ExError {
                source: atoms::entity(),
                reason: e.to_string(),
            }))
        })?;
    }

    Ok(ctx)
}
