use cedar_policy::{Policy, PolicyId};
use rustler::{Error, NifResult, ResourceArc, nif};

use crate::{atoms, error::ExError, state::State};

#[nif]
fn add_policy(
    ctx: ResourceArc<State>,
    policy: &str,
    id: Option<&str>,
) -> NifResult<ResourceArc<State>> {
    let id = id.map_or(None, |v| Some(PolicyId::new(v)));

    let p = Policy::parse(id, policy).map_err(|e| {
        Error::Term(Box::new(ExError {
            source: atoms::policy(),
            reason: e.to_string(),
        }))
    })?;

    add_policy_to_set(ctx, p)
}

#[nif]
fn add_policy_json(
    ctx: ResourceArc<State>,
    policy: &str,
    id: Option<&str>,
) -> NifResult<ResourceArc<State>> {
    let id = id.map_or(None, |v| Some(PolicyId::new(v)));

    let json = serde_json::from_str(policy).map_err(|e| {
        Error::Term(Box::new(ExError {
            source: atoms::json(),
            reason: e.to_string(),
        }))
    })?;

    let p = Policy::from_json(id, json).map_err(|e| {
        Error::Term(Box::new(ExError {
            source: atoms::json(),
            reason: e.to_string(),
        }))
    })?;

    add_policy_to_set(ctx, p)
}

fn add_policy_to_set(ctx: ResourceArc<State>, p: Policy) -> NifResult<ResourceArc<State>> {
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
