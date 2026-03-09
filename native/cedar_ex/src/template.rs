use std::collections::HashMap;

use cedar_policy::{EntityUid, PolicyId, SlotId, Template};
use rustler::{Error, NifResult, NifUnitEnum, ResourceArc, nif};

use crate::{atoms, common::ExEntityUid, error::ExError, state::State};

#[derive(NifUnitEnum, PartialEq, Eq, Hash, Debug)]
pub(crate) enum ExSlotId {
    Principal,
    Resource,
}

#[nif]
pub(crate) fn add_template(
    ctx: ResourceArc<State>,
    template: &str,
    id: Option<&str>,
) -> NifResult<ResourceArc<State>> {
    let t =
        Template::parse(id.map_or(None, |v| Some(PolicyId::new(v))), template).map_err(|e| {
            Error::Term(Box::new(ExError {
                source: atoms::template(),
                reason: e.to_string(),
            }))
        })?;

    {
        // FIXME: Better error handling
        let mut policy_set = ctx.policy_set.write().unwrap();
        policy_set.add_template(t).map_err(|e| {
            Error::Term(Box::new(ExError {
                source: atoms::template(),
                reason: e.to_string(),
            }))
        })?;
    }

    Ok(ctx)
}

#[nif]
pub(crate) fn link(
    ctx: ResourceArc<State>,
    template_id: &str,
    policy_id: &str,
    values: HashMap<ExSlotId, ExEntityUid>,
) -> NifResult<ResourceArc<State>> {
    let v = values.into_iter().fold(Ok(HashMap::new()), |acc, (k, v)| {
        let mut map: HashMap<SlotId, EntityUid> = acc?;
        let entity_uid: NifResult<EntityUid> = v.into();
        match k {
            ExSlotId::Principal => map.insert(SlotId::principal(), entity_uid?),
            ExSlotId::Resource => map.insert(SlotId::resource(), entity_uid?),
        };
        Ok(map)
    })?;

    {
        // FIXME: Better error handling
        let mut policy_set = ctx.policy_set.write().unwrap();
        policy_set
            .link(PolicyId::new(template_id), PolicyId::new(policy_id), v)
            .map_err(|e| {
                Error::Term(Box::new(ExError {
                    source: atoms::template(),
                    reason: e.to_string(),
                }))
            })?;
    }

    Ok(ctx)
}
