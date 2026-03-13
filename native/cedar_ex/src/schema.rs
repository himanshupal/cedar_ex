use cedar_policy::{Schema, ValidationMode, Validator};
use rustler::{Error, NifResult, ResourceArc, nif};

use crate::{atoms, common::ExFormat, error::ExError, state::State};

#[nif]
pub(crate) fn validate(
    ctx: ResourceArc<State>,
    schema: ExFormat,
    strict: bool,
) -> NifResult<ResourceArc<State>> {
    let s = parse_schema(Some(schema))?.unwrap();

    {
        // FIXME: Better error handling
        let policy_set = ctx.policy_set.read().unwrap();
        let result = Validator::new(s).validate(&policy_set, ValidationMode::default());

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

pub(crate) fn parse_schema(schema: Option<ExFormat>) -> NifResult<Option<Schema>> {
    schema.map_or(Ok(None), |v| match v {
        ExFormat::Cedar(value) => {
            let (s, warnings) = Schema::from_cedarschema_str(value).map_err(|e| {
                Error::Term(Box::new(ExError {
                    source: atoms::schema(),
                    reason: e.to_string(),
                }))
            })?;
            for warning in warnings {
                println!("SCHEMA_WARNING: {}", warning);
            }
            Ok(Some(s))
        }
        ExFormat::Json(value) => {
            let s = Schema::from_json_str(value).map_err(|e| {
                Error::Term(Box::new(ExError {
                    source: atoms::schema(),
                    reason: e.to_string(),
                }))
            })?;
            Ok(Some(s))
        }
    })
}
