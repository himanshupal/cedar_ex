use cedar_policy::{Schema, ValidationMode, Validator};
use rustler::{Error as NifError, NifResult, NifStruct, ResourceArc, nif};
use std::error::Error;

use crate::{common::ExFormat, error::ExError, state::State};

#[derive(NifStruct, Debug)]
#[module = "CedarPolicy.SchemaValidationResult"]
struct SchemaValidationResult {
    passed: bool,
    errors: Vec<String>,
    warnings: Vec<String>,
    passed_without_warnings: bool,
}

impl From<cedar_policy::ValidationResult> for SchemaValidationResult {
    fn from(result: cedar_policy::ValidationResult) -> Self {
        SchemaValidationResult {
            passed: result.validation_passed(),
            errors: result
                .validation_errors()
                .map(|e| e.source().unwrap_or(&e).to_string())
                .collect(),
            warnings: result
                .validation_warnings()
                .map(|w| w.source().unwrap_or(&w).to_string())
                .collect(),
            passed_without_warnings: result.validation_passed_without_warnings(),
        }
    }
}

#[nif(name = "validate_schema")]
pub fn validate(
    state: ResourceArc<State>,
    schema: ExFormat,
    strict: bool,
) -> NifResult<SchemaValidationResult> {
    let s = parse_schema(Some(schema))?.ok_or(NifError::BadArg)?;
    let policy_set = state
        .policy_set
        .read()
        .map_err(|e| ExError::from(e).into())?;

    let validation_mode = if strict {
        ValidationMode::Strict
    } else {
        ValidationMode::default()
    };

    let result = Validator::new(s).validate(&policy_set, validation_mode);
    Ok(SchemaValidationResult::from(result))
}

pub(crate) fn parse_schema(schema: Option<ExFormat>) -> NifResult<Option<Schema>> {
    schema.map_or(Ok(None), |v| match v {
        ExFormat::Cedar(value) => {
            let (s, warnings) =
                Schema::from_cedarschema_str(value).map_err(|e| ExError::from(e).into())?;
            // FIXME: Find a better way to send this to user
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
