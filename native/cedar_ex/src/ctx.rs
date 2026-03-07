use cedar_policy::{
    Context as CedarContext, EntityId, EntityTypeName, EntityUid, Policy, Request,
    RequestValidationError, RestrictedExpression, Schema,
};
use rustler::{
    Encoder, Env, Error, NifResult, NifStruct, NifTaggedEnum, Resource, ResourceArc, Term, nif,
};
use std::{str::FromStr, sync::Mutex};

use crate::atoms;

pub struct Context {
    pub policy: Mutex<Policy>,
}

impl Resource for Context {
    const IMPLEMENTS_DESTRUCTOR: bool = true;

    fn destructor(self, _: Env<'_>) {
        println!("Dropping context resource... {:?}", atoms::ok())
    }
}

pub fn on_load(env: Env, _: Term) -> bool {
    env.register::<Context>().is_ok()
}

#[derive(NifStruct)]
#[module = "CedarPolicy.EntityUid"]
struct ExEntityUid<'a> {
    type_name: &'a str,
    id: &'a str,
}

impl<'a> ExEntityUid<'a> {
    fn to_entity_uid(self) -> EntityUid {
        EntityUid::from_type_name_and_id(
            EntityTypeName::from_str(self.type_name).unwrap(),
            EntityId::from_str(self.id).unwrap(),
        )
    }
}

#[derive(NifTaggedEnum)]
enum ExRestrictedExpression<'a> {
    Long(i64),
    Bool(bool),
    Ip(String),
    String(String),
    Decimal(String),
    DateTime(String),
    Duration(String),
    EntityUid(ExEntityUid<'a>),
    Set(Vec<ExRestrictedExpression<'a>>),
    Record(Vec<(String, ExRestrictedExpression<'a>)>),
}

impl<'a> ExRestrictedExpression<'a> {
    fn to_restricted_expression(self) -> RestrictedExpression {
        match self {
            ExRestrictedExpression::Ip(value) => RestrictedExpression::new_ip(value),
            ExRestrictedExpression::Bool(value) => RestrictedExpression::new_bool(value),
            ExRestrictedExpression::Long(value) => RestrictedExpression::new_long(value),
            ExRestrictedExpression::String(value) => RestrictedExpression::new_string(value),
            ExRestrictedExpression::Decimal(value) => RestrictedExpression::new_decimal(value),
            ExRestrictedExpression::DateTime(value) => RestrictedExpression::new_datetime(value),
            ExRestrictedExpression::Duration(value) => RestrictedExpression::new_duration(value),
            ExRestrictedExpression::Set(value) => RestrictedExpression::new_set(
                value.into_iter().map(|v| v.to_restricted_expression()),
            ),
            ExRestrictedExpression::EntityUid(value) => {
                RestrictedExpression::new_entity_uid(value.to_entity_uid())
            }
            ExRestrictedExpression::Record(value) => RestrictedExpression::new_record(
                value
                    .into_iter()
                    .map(|(k, v)| (k, v.to_restricted_expression())),
            )
            .unwrap(),
        }
    }
}

#[nif]
fn new_context(p: &str) -> NifResult<ResourceArc<Context>> {
    match Policy::parse(None, p) {
        Ok(policy) => Ok(ResourceArc::new(Context {
            policy: Mutex::new(policy),
        })),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(Error::Atom("error"))
        }
    }
}

#[nif]
fn get_policy_as_json(ctx: ResourceArc<Context>) -> impl Encoder {
    let policy = ctx.policy.lock().unwrap();
    policy.to_json().unwrap().to_string()
}

#[nif]
fn create_request<'a>(
    p: ExEntityUid,
    a: ExEntityUid,
    r: ExEntityUid,
    // TODO: Support other types for RE & schema
    c: Vec<(String, ExRestrictedExpression)>,
    s: Option<&'a str>,
) -> impl Encoder {
    let p = p.to_entity_uid();
    let a = a.to_entity_uid();
    let r = r.to_entity_uid();
    let c = CedarContext::from_pairs(
        c.into_iter()
            .map(|(k, v)| (k, v.to_restricted_expression())),
    )
    .unwrap();

    let s = if let Some(value) = s {
        Some(&Schema::from_cedarschema_str(value).unwrap().0)
    } else {
        None
    };

    match Request::new(p, a, r, c, s) {
        Ok(_req) => {}
        Err(e) => {
            match e {
                RequestValidationError::UndeclaredAction(undeclared_action_error) => {
                    eprintln!(
                        "Action {} not found",
                        undeclared_action_error.action().to_json_value().unwrap()
                    )
                }
                RequestValidationError::UndeclaredPrincipalType(
                    undeclared_principal_type_error,
                ) => {
                    eprintln!(
                        "Principal {} not found",
                        undeclared_principal_type_error.principal_ty()
                    )
                }
                RequestValidationError::UndeclaredResourceType(undeclared_resource_type_error) => {
                    eprintln!(
                        "Resource {} not found",
                        undeclared_resource_type_error.resource_ty()
                    )
                }
                RequestValidationError::InvalidPrincipalType(invalid_principal_type_error) => {
                    eprintln!(
                        "Invalid principal {}",
                        invalid_principal_type_error.principal_ty()
                    )
                }
                RequestValidationError::InvalidResourceType(invalid_resource_type_error) => {
                    eprintln!(
                        "Invalid resource type {}",
                        invalid_resource_type_error.resource_ty()
                    )
                }
                RequestValidationError::InvalidContext(invalid_context_error) => {
                    eprintln!("Invalid context {}", invalid_context_error.context())
                }
                RequestValidationError::TypeOfContext(type_of_context_error) => {
                    eprintln!("Invalid type of context {}", type_of_context_error)
                }
                RequestValidationError::InvalidEnumEntity(invalid_enum_entity_error) => {
                    eprintln!("Invalid enum {}", invalid_enum_entity_error)
                }
                _ => todo!(),
            };
        }
    };
}
