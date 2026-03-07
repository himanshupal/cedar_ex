use cedar_policy::{
    Authorizer, Context, Entities, Entity, EntityId, EntityTypeName, EntityUid, Policy, PolicyId,
    PolicySet, Request, RequestValidationError, RestrictedExpression, Schema, ValidationMode,
    Validator, entities_errors::EntitiesError,
};
use rustler::{Env, Error, NifResult, NifStruct, NifTaggedEnum, Resource, ResourceArc, Term, nif};
use std::{str::FromStr, sync::RwLock};

use crate::atoms;

type RecordItems = Vec<(String, ExRestrictedExpression)>;

pub struct State {
    pub entities: RwLock<Entities>,
    pub policies: RwLock<PolicySet>,
}

impl Resource for State {
    const IMPLEMENTS_DESTRUCTOR: bool = true;

    fn destructor(self, _: Env<'_>) {
        println!("Dropping context resource... {:?}", atoms::ok())
    }
}

pub fn on_load(env: Env, _: Term) -> bool {
    env.register::<State>().is_ok()
}

#[derive(NifStruct, Debug)]
#[module = "CedarPolicy.EntityUid"]
struct ExEntityUid {
    type_name: String,
    id: String,
}

impl ExEntityUid {
    fn to_entity_uid(self) -> EntityUid {
        EntityUid::from_type_name_and_id(
            EntityTypeName::from_str(&self.type_name).unwrap(),
            EntityId::from_str(&self.id).unwrap(),
        )
    }
}

#[derive(NifStruct, Debug)]
#[module = "CedarPolicy.Entity"]
struct ExEntity {
    id: ExEntityUid,
    tags: RecordItems,
    attrs: RecordItems,
    parents: Vec<ExEntityUid>,
}

#[derive(NifTaggedEnum, Debug)]
enum ExRestrictedExpression {
    Long(i64),
    Bool(bool),
    Ip(String),
    String(String),
    Decimal(String),
    DateTime(String),
    Duration(String),
    Record(RecordItems),
    EntityUid(ExEntityUid),
    Set(Vec<ExRestrictedExpression>),
}

impl ExRestrictedExpression {
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
fn new() -> ResourceArc<State> {
    ResourceArc::new(State {
        entities: RwLock::new(Entities::empty()),
        policies: RwLock::new(PolicySet::new()),
    })
}

#[nif]
fn add_policy<'a>(
    ctx: ResourceArc<State>,
    policy: &'a str,
    id: Option<&'a str>,
) -> NifResult<ResourceArc<State>> {
    match Policy::parse(id.map_or(None, |v| Some(PolicyId::new(v))), policy) {
        Ok(p) => {
            if let Ok(mut policies) = ctx.policies.write() {
                policies.add(p).unwrap();
            }
            Ok(ctx)
        }
        Err(_e) => Err(Error::BadArg),
    }
}

#[nif]
fn add_entities<'a>(
    ctx: ResourceArc<State>,
    entities: Vec<ExEntity>,
    schema: Option<&str>,
) -> NifResult<ResourceArc<State>> {
    let e = entities.into_iter().map(|entity| {
        let attrs = entity
            .attrs
            .into_iter()
            .map(|(k, v)| (k, v.to_restricted_expression()));

        let tags = entity
            .tags
            .into_iter()
            .map(|(k, v)| (k, v.to_restricted_expression()));

        let parents = entity.parents.into_iter().map(|e| e.to_entity_uid());

        Entity::new_with_tags(entity.id.to_entity_uid(), attrs, parents, tags).unwrap()
    });

    if let Ok(mut entities) = ctx.entities.write() {
        let current = entities.clone();
        let schema = schema.map_or(None, |s| parse_schema(s));
        match current.add_entities(e, schema.as_ref()) {
            Ok(new_entities) => {
                *entities = new_entities;
            }
            Err(e) => handle_add_entity_error(e),
        }
    }

    Ok(ctx)
}

#[nif]
fn validate<'a>(ctx: ResourceArc<State>, schema: &'a str) -> NifResult<bool> {
    if let Some(s) = parse_schema(schema) {
        let policies = ctx.policies.read().unwrap();
        let result = Validator::new(s).validate(&policies, ValidationMode::default());

        for warning in result.validation_warnings() {
            println!("WARNING: {}", warning);
        }

        for error in result.validation_errors() {
            eprintln!("ERROR: {}", error);
        }

        Ok(result.validation_passed())
    } else {
        Ok(false)
    }
}

#[nif]
fn create_request<'a>(
    ctx: ResourceArc<State>,
    p: ExEntityUid,
    a: ExEntityUid,
    r: ExEntityUid,
    // TODO: Support other types for RE & schema
    c: RecordItems,
    s: Option<&str>,
) -> NifResult<bool> {
    let p = p.to_entity_uid();
    let a = a.to_entity_uid();
    let r = r.to_entity_uid();
    let c = Context::from_pairs(
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
        Ok(r) => {
            let response = Authorizer::new().is_authorized(
                &r,
                &*ctx.policies.read().unwrap(),
                &*ctx.entities.read().unwrap(),
            );

            let diagnostics = response.diagnostics();

            for err in diagnostics.errors() {
                println!("Error: {}", err);
            }

            for reason in diagnostics.reason() {
                println!("Reason: {}", reason);
            }

            match response.decision() {
                cedar_policy::Decision::Allow => Ok(true),
                cedar_policy::Decision::Deny => Ok(false),
            }
        }
        Err(e) => {
            handle_request_error(e);
            Err(Error::Atom("duck"))
        }
    }
}

fn parse_schema(schema: &str) -> Option<Schema> {
    match Schema::from_cedarschema_str(schema) {
        Ok((s, warnings)) => {
            for warning in warnings {
                println!("SCHEMA_WARNING: {}", warning);
            }
            Some(s)
        }
        Err(e) => match e {
            cedar_policy::CedarSchemaError::Io(e) => {
                eprintln!("IO failed: {}", e);
                None
            }
            cedar_policy::CedarSchemaError::Parse(e) => {
                eprintln!("Parsing failed: {}", e);
                None
            }
            _ => todo!(),
        },
    }
}

fn handle_request_error(e: RequestValidationError) {
    match e {
        RequestValidationError::UndeclaredAction(undeclared_action_error) => {
            eprintln!(
                "Action {} not found",
                undeclared_action_error.action().to_json_value().unwrap()
            )
        }
        RequestValidationError::UndeclaredPrincipalType(undeclared_principal_type_error) => {
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

fn handle_add_entity_error(e: EntitiesError) {
    match e {
        EntitiesError::Serialization(json_serialization_error) => {
            eprint!("Serialization error: {}", json_serialization_error);
        }
        EntitiesError::Deserialization(json_deserialization_error) => {
            eprint!("Deserialization error: {}", json_deserialization_error);
        }
        EntitiesError::Duplicate(duplicate) => {
            eprint!("Duplicate error: {}", duplicate);
        }
        EntitiesError::TransitiveClosureError(transitive_closure_error) => {
            eprint!("TransitiveClosureError error: {}", transitive_closure_error);
        }
        EntitiesError::InvalidEntity(entity_schema_conformance_error) => {
            eprint!("InvalidEntity error: {}", entity_schema_conformance_error);
        }
    }
}
