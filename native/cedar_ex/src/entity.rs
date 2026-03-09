use cedar_policy::{Entity, EntityUid};
use rustler::{Error, NifResult, NifStruct, ResourceArc, nif};

use crate::{
    atoms,
    common::{ExEntityUid, ExRecordItem, ExRecordItems, RecordItems},
    error::ExError,
    schema::parse_schema,
    state::State,
};

#[derive(NifStruct, Debug)]
#[module = "CedarPolicy.Entity"]
pub(crate) struct ExEntity {
    id: ExEntityUid,
    tags: Vec<ExRecordItem>,
    attrs: Vec<ExRecordItem>,
    parents: Vec<ExEntityUid>,
}

#[nif]
pub(crate) fn add_entities(
    ctx: ResourceArc<State>,
    entities: Vec<ExEntity>,
    schema: Option<&str>,
) -> NifResult<ResourceArc<State>> {
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

    {
        // FIXME: Better error handling
        let mut entities = ctx.entities.write().unwrap();

        let current = entities.clone();
        let schema = parse_schema(schema);

        *entities = current.add_entities(e, schema.as_ref()).map_err(|e| {
            Error::Term(Box::new(ExError {
                source: atoms::entity(),
                reason: e.to_string(),
            }))
        })?;
    }

    Ok(ctx)
}
