use cedar_policy::{Schema, ValidationMode, Validator};
use rustler::{NifResult, ResourceArc, nif};

use crate::state::State;

#[nif]
pub(crate) fn validate<'a>(ctx: ResourceArc<State>, schema: &'a str) -> NifResult<bool> {
    if let Some(s) = parse_schema(schema) {
        // FIXME: Better error handling
        let policies = ctx.policies.read().unwrap();
        let result = Validator::new(s).validate(&policies, ValidationMode::default());

        for warning in result.validation_warnings() {
            println!("VALIDATION_WARNING: {}", warning);
        }

        for error in result.validation_errors() {
            eprintln!("VALIDATION_ERROR: {}", error);
        }

        Ok(result.validation_passed())
    } else {
        Ok(false)
    }
}

pub(crate) fn parse_schema(schema: &str) -> Option<Schema> {
    match Schema::from_cedarschema_str(schema) {
        Ok((s, warnings)) => {
            for warning in warnings {
                println!("SCHEMA_WARNING: {}", warning);
            }
            Some(s)
        }
        Err(e) => match e {
            cedar_policy::CedarSchemaError::Io(e) => {
                eprintln!("SCHEMA_IO_ERROR: {}", e);
                None
            }
            cedar_policy::CedarSchemaError::Parse(e) => {
                eprintln!("SCHEMA_PARSING_ERROR: {}", e);
                None
            }
            _ => todo!(),
        },
    }
}
