use cedar_policy::{Schema, ValidationMode, Validator};
use rustler::{Error, NifResult, ResourceArc, nif};

use crate::{atoms, error::ExError, state::State};

#[nif]
pub(crate) fn validate(
    ctx: ResourceArc<State>,
    schema: &str,
    strict: Option<bool>,
) -> NifResult<ResourceArc<State>> {
    match parse_schema(Some(schema))? {
        None => Ok(ctx),
        Some(s) => {
            let strict_check = strict.unwrap_or(false);
            validate_schema(ctx, s, strict_check)
        }
    }
}

#[nif]
pub(crate) fn validate_json(
    ctx: ResourceArc<State>,
    schema: &str,
    strict: Option<bool>,
) -> NifResult<ResourceArc<State>> {
    match parse_schema_json(Some(schema))? {
        None => Ok(ctx),
        Some(s) => {
            let strict_check = strict.unwrap_or(false);
            validate_schema(ctx, s, strict_check)
        }
    }
}

fn validate_schema(
    ctx: ResourceArc<State>,
    schema: Schema,
    strict: bool,
) -> NifResult<ResourceArc<State>> {
    {
        // FIXME: Better error handling
        let policy_set = ctx.policy_set.read().unwrap();
        let result = Validator::new(schema).validate(&policy_set, ValidationMode::default());

        for error in result.validation_errors() {
            return Err(Error::Term(Box::new(ExError {
                source: atoms::schema(),
                reason: error.to_string(),
            })));
        }

        for warning in result.validation_warnings() {
            println!("VALIDATION_WARNING: {}", warning);
            if strict {
                return Err(Error::Term(Box::new(ExError {
                    source: atoms::schema(),
                    reason: warning.to_string(),
                })));
            }
        }
    }

    Ok(ctx)
}

pub(crate) fn parse_schema(schema: Option<&str>) -> NifResult<Option<Schema>> {
    schema.map_or(Ok(None), |v| {
        let (s, warnings) = Schema::from_cedarschema_str(v).map_err(|e| {
            Error::Term(Box::new(ExError {
                source: atoms::schema(),
                reason: e.to_string(),
            }))
        })?;
        for warning in warnings {
            println!("SCHEMA_WARNING: {}", warning);
        }
        Ok(Some(s))
    })
}

pub(crate) fn parse_schema_json(schema: Option<&str>) -> NifResult<Option<Schema>> {
    schema.map_or(Ok(None), |v| {
        let s = Schema::from_json_str(v).map_err(|e| {
            Error::Term(Box::new(ExError {
                source: atoms::schema(),
                reason: e.to_string(),
            }))
        })?;
        Ok(Some(s))
    })
}
