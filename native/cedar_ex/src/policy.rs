use cedar_policy::{Context, EntityUid, Policy, PolicyId, Request};
use rustler::{Error, NifResult, ResourceArc, nif};

use crate::{
    atoms,
    common::{ExEntityUid, ExRecordItem, ExRecordItems, RecordItems},
    error::ExError,
    schema::parse_schema,
    state::State,
};

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

#[nif]
fn verify(
    ctx: ResourceArc<State>,
    p: ExEntityUid,
    a: ExEntityUid,
    r: ExEntityUid,
    // TODO: Support other types for RE & schema
    c: Vec<ExRecordItem>,
    s: Option<&str>,
) -> NifResult<bool> {
    let p: NifResult<EntityUid> = p.into();
    let a: NifResult<EntityUid> = a.into();
    let r: NifResult<EntityUid> = r.into();

    let cx: NifResult<RecordItems> = ExRecordItems(c).into();
    let c = Context::from_pairs(cx?).map_err(|e| {
        Error::Term(Box::new(ExError {
            source: atoms::context(),
            reason: e.to_string(),
        }))
    });

    let s = parse_schema(s)?;

    let rq = Request::new(p?, a?, r?, c?, s.as_ref()).map_err(|e| {
        Error::Term(Box::new(ExError {
            source: atoms::request(),
            reason: e.to_string(),
        }))
    })?;

    let authorizer = &*ctx.authorizer.read().unwrap();
    let response = authorizer.is_authorized(
        &rq,
        // FIXME: Better error handling
        &*ctx.policy_set.read().unwrap(),
        &*ctx.entities.read().unwrap(),
    );

    let diagnostics = response.diagnostics();

    for error in diagnostics.errors() {
        eprintln!("VERIFICATION_ERROR: {}", error);
        return Err(Error::Term(Box::new(ExError {
            source: atoms::request(),
            reason: error.to_string(),
        })));
    }

    for reason in diagnostics.reason() {
        println!("Reason: {:?}", reason);
    }

    match response.decision() {
        cedar_policy::Decision::Allow => Ok(true),
        cedar_policy::Decision::Deny => Ok(false),
    }
}
