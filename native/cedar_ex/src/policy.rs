use cedar_policy::{Policy, PolicyId};
use rustler::{Error, NifResult, ResourceArc, nif};

use crate::{atoms, common::ExFormat, error::ExError, state::State};

#[nif]
fn add_policy(
    ctx: ResourceArc<State>,
    policy: ExFormat,
    id: Option<&str>,
) -> NifResult<ResourceArc<State>> {
    let id = id.map_or(None, |v| Some(PolicyId::new(v)));

    let p = match policy {
        ExFormat::Cedar(value) => Policy::parse(id, value).map_err(|e| {
            Error::Term(Box::new(ExError {
                source: atoms::policy(),
                reason: e.to_string(),
            }))
        }),
        ExFormat::Json(value) => {
            let json = serde_json::from_str(value).map_err(|e| {
                Error::Term(Box::new(ExError {
                    source: atoms::json(),
                    reason: e.to_string(),
                }))
            })?;

            Policy::from_json(id, json).map_err(|e| {
                Error::Term(Box::new(ExError {
                    source: atoms::json(),
                    reason: e.to_string(),
                }))
            })
        }
    }?;

    {
        // FIXME: Better error handling
        let mut policy_set = ctx.policy_set.write().unwrap();
        policy_set.add(p).map_err(|e| {
            Error::Term(Box::new(ExError {
                source: atoms::policy(),
                reason: e.to_string(),
            }))
        })?;
    }

    Ok(ctx)
}
