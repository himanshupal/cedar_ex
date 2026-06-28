use cedar_policy::{
    CedarSchemaError, ContextCreationError, Entities, EntityAttrEvaluationError, EvaluationError,
    ExpressionConstructionError, ParseErrors, PolicyFromJsonError, PolicySet, PolicySetError,
    RequestValidationError, SchemaError, entities_errors::EntitiesError,
};
use rustler::{Atom, Error as NifError, NifStruct, types::atom};
use std::{
    error::Error,
    fmt::Display,
    sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

use crate::atoms;

#[derive(NifStruct, Debug)]
#[module = "CedarPolicy.Error"]
pub(crate) struct ExError {
    source: Atom,
    reason: String,
}

impl Into<NifError> for ExError {
    fn into(self) -> NifError {
        NifError::Term(Box::new(self))
    }
}

impl From<PoisonError<RwLockReadGuard<'_, PolicySet>>> for ExError {
    fn from(e: PoisonError<RwLockReadGuard<'_, PolicySet>>) -> ExError {
        ExError {
            source: atom::nif_panicked(),
            reason: e.to_string(),
        }
    }
}

impl From<PoisonError<RwLockReadGuard<'_, Entities>>> for ExError {
    fn from(e: PoisonError<RwLockReadGuard<'_, Entities>>) -> ExError {
        ExError {
            source: atom::nif_panicked(),
            reason: e.to_string(),
        }
    }
}

impl From<PoisonError<RwLockWriteGuard<'_, PolicySet>>> for ExError {
    fn from(e: PoisonError<RwLockWriteGuard<'_, PolicySet>>) -> ExError {
        ExError {
            source: atom::nif_panicked(),
            reason: e.to_string(),
        }
    }
}

impl From<PoisonError<RwLockWriteGuard<'_, Entities>>> for ExError {
    fn from(e: PoisonError<RwLockWriteGuard<'_, Entities>>) -> ExError {
        ExError {
            source: atom::nif_panicked(),
            reason: e.to_string(),
        }
    }
}

impl From<PolicySetError> for ExError {
    fn from(e: PolicySetError) -> Self {
        match e {
            PolicySetError::AlreadyDefined(e) => ExError {
                source: atoms::template(),
                reason: format!("AlreadyDefined: {}", e),
            },
            PolicySetError::Linking(e) => ExError {
                source: atoms::template(),
                reason: format!("Linking: {}", e.source().ok_or(&e).unwrap()),
            },
            PolicySetError::ExpectedStatic(e) => ExError {
                source: atoms::template(),
                reason: format!("ExpectedStatic: {}", e),
            },
            PolicySetError::ExpectedTemplate(e) => ExError {
                source: atoms::template(),
                reason: format!("ExpectedTemplate: {}", e),
            },
            PolicySetError::PolicyNonexistent(e) => ExError {
                source: atoms::template(),
                reason: format!("PolicyNonexistent: {}", e),
            },
            PolicySetError::TemplateNonexistent(e) => ExError {
                source: atoms::template(),
                reason: format!("TemplateNonexistent: {}", e),
            },
            PolicySetError::RemoveTemplateWithActiveLinks(e) => ExError {
                source: atoms::template(),
                reason: format!("RemoveTemplateWithActiveLinks: {}", e),
            },
            PolicySetError::RemoveTemplateNotTemplate(e) => ExError {
                source: atoms::template(),
                reason: format!("RemoveTemplateNotTemplate: {}", e),
            },
            PolicySetError::LinkNonexistent(e) => ExError {
                source: atoms::template(),
                reason: format!("LinkNonexistent: {}", e),
            },
            PolicySetError::UnlinkLinkNotLink(e) => ExError {
                source: atoms::template(),
                reason: format!("UnlinkLinkNotLink: {}", e),
            },
            PolicySetError::FromJson(e) => ExError {
                source: atoms::template(),
                reason: format!("FromJson: {}", e),
            },
            PolicySetError::ToJson(e) => ExError {
                source: atoms::template(),
                reason: format!("ToJson: {}", e),
            },
            PolicySetError::JsonPolicySet(e) => ExError {
                source: atoms::template(),
                reason: format!("JsonPolicySet: {}", e),
            },
            PolicySetError::PstConversion(e) => ExError {
                source: atoms::template(),
                reason: format!("PstConversion: {}", e),
            },
            _ => ExError {
                source: atoms::template(),
                reason: format!("PolicySetError: {}", e),
            },
        }
    }
}

impl From<SchemaError> for ExError {
    fn from(e: SchemaError) -> Self {
        match e {
            SchemaError::ActionEntityTypeDeclared(e) => ExError {
                source: atoms::schema(),
                reason: format!("ActionEntityTypeDeclared: {}", e),
            },
            SchemaError::JsonSerialization(e) => ExError {
                source: atoms::schema(),
                reason: format!("JsonSerialization: {}", e),
            },
            SchemaError::JsonDeserialization(e) => ExError {
                source: atoms::schema(),
                reason: format!("JsonDeserialization: {}", e),
            },
            SchemaError::ActionTransitiveClosure(e) => ExError {
                source: atoms::schema(),
                reason: format!("ActionTransitiveClosure: {}", e),
            },
            SchemaError::EntityTypeTransitiveClosure(e) => ExError {
                source: atoms::schema(),
                reason: format!("EntityTypeTransitiveClosure: {}", e),
            },
            SchemaError::UnsupportedFeature(e) => ExError {
                source: atoms::schema(),
                reason: format!("UnsupportedFeature: {}", e),
            },
            SchemaError::UndeclaredEntityTypes(e) => ExError {
                source: atoms::schema(),
                reason: format!("UndeclaredEntityTypes: {}", e),
            },
            SchemaError::TypeNotDefined(e) => ExError {
                source: atoms::schema(),
                reason: format!("TypeNotDefined: {}", e),
            },
            SchemaError::ActionNotDefined(e) => ExError {
                source: atoms::schema(),
                reason: format!("ActionNotDefined: {}", e),
            },
            SchemaError::TypeShadowing(e) => ExError {
                source: atoms::schema(),
                reason: format!("TypeShadowing: {}", e),
            },
            SchemaError::ActionShadowing(e) => ExError {
                source: atoms::schema(),
                reason: format!("ActionShadowing: {}", e),
            },
            SchemaError::DuplicateEntityType(e) => ExError {
                source: atoms::schema(),
                reason: format!("DuplicateEntityType: {}", e),
            },
            SchemaError::DuplicateAction(e) => ExError {
                source: atoms::schema(),
                reason: format!("DuplicateAction: {}", e),
            },
            SchemaError::DuplicateCommonType(e) => ExError {
                source: atoms::schema(),
                reason: format!("DuplicateCommonType: {}", e),
            },
            SchemaError::CycleInActionHierarchy(e) => ExError {
                source: atoms::schema(),
                reason: format!("CycleInActionHierarchy: {}", e),
            },
            SchemaError::CycleInCommonTypeReferences(e) => ExError {
                source: atoms::schema(),
                reason: format!("CycleInCommonTypeReferences: {}", e),
            },
            SchemaError::ContextOrShapeNotRecord(e) => ExError {
                source: atoms::schema(),
                reason: format!("ContextOrShapeNotRecord: {}", e),
            },
            SchemaError::UnknownExtensionType(e) => ExError {
                source: atoms::schema(),
                reason: format!("UnknownExtensionType: {}", e),
            },
            SchemaError::ReservedName(e) => ExError {
                source: atoms::schema(),
                reason: format!("ReservedName: {}", e),
            },
            SchemaError::CommonTypeInvariantViolation(e) => ExError {
                source: atoms::schema(),
                reason: format!("CommonTypeInvariantViolation: {}", e),
            },
            SchemaError::ActionInvariantViolation(e) => ExError {
                source: atoms::schema(),
                reason: format!("ActionInvariantViolation: {}", e),
            },
            _ => ExError {
                source: atoms::schema(),
                reason: format!("SchemaError: {}", e),
            },
        }
    }
}

impl From<CedarSchemaError> for ExError {
    fn from(e: CedarSchemaError) -> Self {
        match e {
            CedarSchemaError::Io(e) => ExError {
                source: atoms::schema(),
                reason: format!("Io: {}", e),
            },
            CedarSchemaError::Parse(e) => ExError {
                source: atoms::schema(),
                reason: format!("Parse: {}", e),
            },
            CedarSchemaError::Schema(e) => ExError {
                source: atoms::schema(),
                reason: format!("Schema: {}", e),
            },
            _ => ExError {
                source: atoms::schema(),
                reason: format!("SchemaError: {}", e),
            },
        }
    }
}

impl From<EntitiesError> for ExError {
    fn from(e: EntitiesError) -> Self {
        match e {
            EntitiesError::Serialization(e) => ExError {
                source: atoms::entity(),
                reason: format!("Serialization: {}", e),
            },
            EntitiesError::Deserialization(e) => ExError {
                source: atoms::entity(),
                reason: format!("Deserialization: {}", e),
            },
            EntitiesError::Duplicate(e) => ExError {
                source: atoms::entity(),
                reason: format!("Duplicate: {}", e),
            },
            EntitiesError::TransitiveClosureError(e) => ExError {
                source: atoms::entity(),
                reason: format!("TransitiveClosureError: {}", e),
            },
            EntitiesError::InvalidEntity(e) => ExError {
                source: atoms::entity(),
                reason: format!("InvalidEntity: {}", e),
            },
        }
    }
}

impl From<EntityAttrEvaluationError> for ExError {
    fn from(e: EntityAttrEvaluationError) -> Self {
        // FIXME: Improve it

        match e.inner() {
            EvaluationError::EntityDoesNotExist(e) => ExError {
                source: atoms::entity(),
                reason: format!("EntityDoesNotExist: {}", e),
            },
            EvaluationError::EntityAttrDoesNotExist(e) => ExError {
                source: atoms::entity(),
                reason: format!("EntityAttrDoesNotExist: {}", e),
            },
            EvaluationError::RecordAttrDoesNotExist(e) => ExError {
                source: atoms::entity(),
                reason: format!("RecordAttrDoesNotExist: {}", e),
            },
            EvaluationError::FailedExtensionFunctionLookup(e) => ExError {
                source: atoms::entity(),
                reason: format!("FailedExtensionFunctionLookup: {}", e),
            },
            EvaluationError::TypeError(e) => ExError {
                source: atoms::entity(),
                reason: format!("TypeError: {}", e),
            },
            EvaluationError::WrongNumArguments(e) => ExError {
                source: atoms::entity(),
                reason: format!("WrongNumArguments: {}", e),
            },
            EvaluationError::IntegerOverflow(e) => ExError {
                source: atoms::entity(),
                reason: format!("IntegerOverflow: {}", e),
            },
            EvaluationError::UnlinkedSlot(e) => ExError {
                source: atoms::entity(),
                reason: format!("UnlinkedSlot: {}", e),
            },
            EvaluationError::FailedExtensionFunctionExecution(e) => ExError {
                source: atoms::entity(),
                reason: format!("FailedExtensionFunctionExecution: {}", e),
            },
            EvaluationError::NonValue(e) => ExError {
                source: atoms::entity(),
                reason: format!("NonValue: {}", e),
            },
            EvaluationError::RecursionLimit(e) => ExError {
                source: atoms::entity(),
                reason: format!("RecursionLimit: {}", e),
            },
        }
    }
}

impl From<ContextCreationError> for ExError {
    fn from(e: ContextCreationError) -> Self {
        match e {
            ContextCreationError::NotARecord(e) => ExError {
                source: atoms::context(),
                reason: format!("NotARecord: {}", e),
            },
            ContextCreationError::Evaluation(e) => ExError {
                source: atoms::context(),
                reason: format!("Evaluation: {}", e),
            },
            ContextCreationError::ExpressionConstruction(e) => ExError {
                source: atoms::context(),
                reason: format!("ExpressionConstruction: {}", e),
            },
        }
    }
}

impl From<RequestValidationError> for ExError {
    fn from(e: RequestValidationError) -> Self {
        match e {
            RequestValidationError::UndeclaredAction(e) => ExError {
                source: atoms::request(),
                reason: format!("UndeclaredAction: {}", e),
            },
            RequestValidationError::UndeclaredPrincipalType(e) => ExError {
                source: atoms::request(),
                reason: format!("UndeclaredPrincipalType: {}", e),
            },
            RequestValidationError::UndeclaredResourceType(e) => ExError {
                source: atoms::request(),
                reason: format!("UndeclaredResourceType: {}", e),
            },
            RequestValidationError::InvalidPrincipalType(e) => ExError {
                source: atoms::request(),
                reason: format!("InvalidPrincipalType: {}", e),
            },
            RequestValidationError::InvalidResourceType(e) => ExError {
                source: atoms::request(),
                reason: format!("InvalidResourceType: {}", e),
            },
            RequestValidationError::InvalidContext(e) => ExError {
                source: atoms::request(),
                reason: format!("InvalidContext: {}", e),
            },
            RequestValidationError::TypeOfContext(e) => ExError {
                source: atoms::request(),
                reason: format!("TypeOfContext: {}", e),
            },
            RequestValidationError::InvalidEnumEntity(e) => ExError {
                source: atoms::request(),
                reason: format!("InvalidEnumEntity: {}", e),
            },
            _ => ExError {
                source: atoms::request(),
                reason: format!("RequestValidationError: {}", e),
            },
        }
    }
}

impl From<ParseErrors> for ExError {
    fn from(e: ParseErrors) -> Self {
        ExError {
            source: atoms::parse(),
            reason: format!(
                "ParseErrors: {}, {}",
                join_errors(e.iter()),
                e.source().unwrap_or(&e)
            ),
        }
    }
}

impl From<PolicyFromJsonError> for ExError {
    fn from(e: PolicyFromJsonError) -> Self {
        ExError {
            source: atoms::json(),
            reason: format!("PolicyFromJsonError: {}", e),
        }
    }
}

impl From<ExpressionConstructionError> for ExError {
    fn from(e: ExpressionConstructionError) -> Self {
        match e {
            ExpressionConstructionError::DuplicateKey(e) => ExError {
                source: atoms::restricted_expression(),
                reason: format!("DuplicateKey: {}", e),
            },
        }
    }
}

impl From<serde_json::Error> for ExError {
    fn from(e: serde_json::Error) -> Self {
        ExError {
            source: atoms::json(),
            // TODO: Better handle json details
            reason: format!("JsonParsingFailed: {}:{}", e.column(), e.line()),
        }
    }
}

fn join_errors<T>(value: T) -> String
where
    T: Iterator,
    T::Item: Display,
{
    value.fold(String::new(), |mut a, v| {
        if a.len() > 0 {
            a.push(' ');
        }
        a + &v.to_string()
    })
}
