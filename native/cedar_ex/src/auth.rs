use cedar_policy::{Authorizer, Context, EntityUid, Request};
use rustler::{NifResult, NifStruct, ResourceArc, nif};

use crate::{
    common::{ExEntityUid, ExFormat, ExRecordItem, ExRecordItems, RecordItems},
    error::ExError,
    schema::parse_schema,
    state::State,
};

#[derive(NifStruct, Debug)]
#[module = "CedarPolicy.AuthorizationResult"]
struct AuthorizationResult {
    authorized: bool,
    errors: Vec<String>,
    reasons: Vec<String>,
}

#[nif(name = "is_authorized")]
fn is_authorized(
    state: ResourceArc<State>,
    principal: ExEntityUid,
    action: ExEntityUid,
    resource: ExEntityUid,
    context: Vec<ExRecordItem>,
    schema: Option<ExFormat>,
) -> NifResult<AuthorizationResult> {
    let request = prepare_request(principal, action, resource, context, schema)?;

    let authorizer = Authorizer::new();
    let response = authorizer.is_authorized(
        &request,
        &*state
            .policy_set
            .read()
            .map_err(|e| ExError::from(e).into())?,
        &*state.entities.read().map_err(|e| ExError::from(e).into())?,
    );

    let diagnostics = response.diagnostics();

    Ok(AuthorizationResult {
        authorized: matches!(response.decision(), cedar_policy::Decision::Allow),
        errors: diagnostics.errors().map(|e| e.to_string()).collect(),
        reasons: diagnostics.reason().map(|r| r.to_string()).collect(),
    })
}

fn prepare_request(
    principal: ExEntityUid,
    action: ExEntityUid,
    resource: ExEntityUid,
    context: Vec<ExRecordItem>,
    schema: Option<ExFormat>,
) -> NifResult<Request> {
    let p: NifResult<EntityUid> = principal.into();
    let a: NifResult<EntityUid> = action.into();
    let r: NifResult<EntityUid> = resource.into();

    let s = parse_schema(schema)?;
    let cx: NifResult<RecordItems> = ExRecordItems(context).into();
    let c = Context::from_pairs(cx?).map_err(|e| ExError::from(e).into());
    Request::new(p?, a?, r?, c?, s.as_ref()).map_err(|e| ExError::from(e).into())
}
