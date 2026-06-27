use cedar_policy::{Schema, ValidationMode, Validator};
use rustler::{NifResult, ResourceArc, nif};

use crate::{common::ExFormat, error::ExError, state::State};

#[nif]
pub(crate) fn validate(
    ctx: ResourceArc<State>,
    schema: ExFormat,
    strict: bool,
) -> NifResult<ResourceArc<State>> {
    let s = parse_schema(Some(schema))?.unwrap();

    let validation_mode = if strict {
        ValidationMode::Strict
    } else {
        ValidationMode::default()
    };

    {
        // FIXME: Better error handling
        let policy_set = ctx.policy_set.read().unwrap();
        let result = Validator::new(s).validate(&policy_set, validation_mode);

        for error in result.validation_errors() {
            return Err(ExError::from(error.to_owned()).into());
        }

        for warning in result.validation_warnings() {
            println!("VALIDATION_WARNING: {}", warning);
            // TODO: Improve return type to handle warnings
        }
    }

    Ok(ctx)
}

pub(crate) fn parse_schema(schema: Option<ExFormat>) -> NifResult<Option<Schema>> {
    schema.map_or(Ok(None), |v| match v {
        ExFormat::Cedar(value) => {
            let (s, warnings) =
                Schema::from_cedarschema_str(value).map_err(|e| ExError::from(e).into())?;
            for warning in warnings {
                println!("SCHEMA_WARNING: {}", warning);
            }
            Ok(Some(s))
        }
        ExFormat::Json(value) => {
            let s = Schema::from_json_str(value).map_err(|e| ExError::from(e).into())?;
            Ok(Some(s))
        }
    })
}
