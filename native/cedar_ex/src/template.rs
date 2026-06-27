use cedar_policy::{EntityUid, PolicyId, SlotId, Template};
use rustler::{NifResult, NifUnitEnum, ResourceArc, nif};
use std::collections::HashMap;

use crate::{
    common::{ExEntityUid, ExFormat},
    error::ExError,
    state::State,
};

#[derive(NifUnitEnum, PartialEq, Eq, Hash, Debug)]
pub(crate) enum ExSlotId {
    Principal,
    Resource,
}

#[nif]
pub(crate) fn add_template(
    ctx: ResourceArc<State>,
    template: ExFormat,
    id: Option<&str>,
) -> NifResult<ResourceArc<State>> {
    let id = id.map_or(None, |v| Some(PolicyId::new(v)));

    let t = match template {
        ExFormat::Cedar(value) => Template::parse(id, value).map_err(|e| ExError::from(e).into()),
        ExFormat::Json(value) => {
            let json = serde_json::from_str(value).map_err(|e| ExError::from(e).into())?;

            Template::from_json(id, json).map_err(|e| ExError::from(e).into())
        }
    }?;

    {
        // FIXME: Better error handling
        let mut policy_set = ctx.policy_set.write().unwrap();
        policy_set
            .add_template(t)
            .map_err(|e| ExError::from(e).into())?;
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
            .map_err(|e| ExError::from(e).into())?;
    }

    Ok(ctx)
}
