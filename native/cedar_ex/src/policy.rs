use cedar_policy::{Policy, PolicyId};
use rustler::{NifResult, ResourceArc, nif};

use crate::{common::ExFormat, error::ExError, state::State};

#[nif]
fn add_policy(
    state: ResourceArc<State>,
    policy: ExFormat,
    id: Option<&str>,
) -> NifResult<ResourceArc<State>> {
    let id = id.map_or(None, |v| Some(PolicyId::new(v)));

    let p = match policy {
        ExFormat::Cedar(value) => Policy::parse(id, value).map_err(|e| ExError::from(e).into()),
        ExFormat::Json(value) => {
            let json = serde_json::from_str(value).map_err(|e| ExError::from(e).into())?;
            Policy::from_json(id, json).map_err(|e| ExError::from(e).into())
        }
    }?;

    {
        let mut policy_set = state
            .policy_set
            .write()
            .map_err(|e| ExError::from(e).into())?;
        policy_set.add(p).map_err(|e| ExError::from(e).into())?;
    }

    Ok(state)
}
