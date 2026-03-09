use cedar_policy::{Schema, ValidationMode, Validator};
use rustler::{NifResult, ResourceArc, nif};

use crate::state::State;

#[nif]
pub(crate) fn validate(ctx: ResourceArc<State>, schema: &str) -> NifResult<bool> {
    if let Some(s) = parse_schema(Some(schema)) {
        // FIXME: Better error handling
        let policy_set = ctx.policy_set.read().unwrap();
        let result = Validator::new(s).validate(&policy_set, ValidationMode::default());

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

pub(crate) fn parse_schema(schema: Option<&str>) -> Option<Schema> {
    schema.map_or(None, |v| {
        Schema::from_cedarschema_str(v).map_or(None, |(s, warnings)| {
            for warning in warnings {
                println!("SCHEMA_WARNING: {}", warning);
            }
            Some(s)
        })
    })
}
