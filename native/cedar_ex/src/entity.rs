use cedar_policy::{Entities, Entity, EntityUid, Schema};
use rustler::{Error, NifResult, NifStruct, ResourceArc, nif};

use crate::{
    atoms,
    common::{ExEntityUid, ExRecordItem, ExRecordItems, RecordItems},
    error::ExError,
    schema::{parse_schema, parse_schema_json},
    state::State,
};

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
    entities: Vec<ExEntity>,
    schema: Option<&str>,
) -> NifResult<ResourceArc<State>> {
    let s = parse_schema(schema)?;

    let e = entities
        .into_iter()
        .map(|entity| -> NifResult<Entity> {
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
        .collect::<NifResult<Vec<Entity>>>()?;

    save_entities(ctx, e, s)
}

#[nif]
pub(crate) fn add_entities_json(
    ctx: ResourceArc<State>,
    entities: &str,
    schema: Option<&str>,
) -> NifResult<ResourceArc<State>> {
    let s = parse_schema_json(schema)?;

    let e = Entities::from_json_str(entities, s.as_ref()).map_err(|e| {
        Error::Term(Box::new(ExError {
            source: atoms::entity(),
            reason: e.to_string(),
        }))
    })?;

    save_entities(ctx, e, s)
}

fn save_entities(
    ctx: ResourceArc<State>,
    e: impl IntoIterator<Item = Entity>,
    s: Option<Schema>,
) -> NifResult<ResourceArc<State>> {
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
