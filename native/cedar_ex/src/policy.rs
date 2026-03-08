use cedar_policy::{Authorizer, Context, EntityUid, Policy, PolicyId, Request};
use rustler::{Error, NifResult, ResourceArc, nif};

use crate::{
    atoms,
    common::{ExEntityUid, ExRecordItem, ExRecordItems, RecordItems},
    error::ExError,
    schema::parse_schema,
    state::State,
};

#[nif]
fn add_policy<'a>(
    ctx: ResourceArc<State>,
    policy: &'a str,
    id: Option<&'a str>,
) -> NifResult<ResourceArc<State>> {
    let p = Policy::parse(id.map_or(None, |v| Some(PolicyId::new(v))), policy).map_err(|e| {
        Error::Term(Box::new(ExError {
            source: atoms::policy(),
            reason: e.to_string(),
        }))
    })?;

    {
        // FIXME: Better error handling
        let mut policies = ctx.policies.write().unwrap();
        policies.add(p).map_err(|e| {
            Error::Term(Box::new(ExError {
                source: atoms::policy(),
                reason: e.to_string(),
            }))
        })?;
    }

    Ok(ctx)
}

#[nif]
fn verify<'a>(
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

    let s = s.map_or(None, |v| parse_schema(v));

    let rq = Request::new(p?, a?, r?, c?, s.as_ref()).map_err(|e| {
        Error::Term(Box::new(ExError {
            source: atoms::request(),
            reason: e.to_string(),
        }))
    })?;

    let response = Authorizer::new().is_authorized(
        &rq,
        // FIXME: Better error handling
        &*ctx.policies.read().unwrap(),
        &*ctx.entities.read().unwrap(),
    );

    let diagnostics = response.diagnostics();

    for err in diagnostics.errors() {
        println!("Error: {}", err);
    }

    for reason in diagnostics.reason() {
        println!("Reason: {:?}", reason);
    }

    match response.decision() {
        cedar_policy::Decision::Allow => Ok(true),
        cedar_policy::Decision::Deny => Ok(false),
    }
}
