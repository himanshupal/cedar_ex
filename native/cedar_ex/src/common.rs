use cedar_policy::{EntityId, EntityTypeName, EntityUid, RestrictedExpression};
use rustler::{Error, NifResult, NifStruct, NifTaggedEnum};
use std::{fmt::Display, str::FromStr};

use crate::{atoms, entity::ExEntity, error::ExError};

pub(crate) type RecordItems = Vec<(String, RestrictedExpression)>;

pub(crate) type ExRecordItem = (String, ExRestrictedExpression);

pub(crate) struct ExRecordItems(pub(crate) Vec<ExRecordItem>);

impl Iterator for ExRecordItems {
    type Item = ExRecordItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}

impl Into<NifResult<RecordItems>> for ExRecordItems {
    fn into(self) -> NifResult<RecordItems> {
        Ok(self
            .0
            .into_iter()
            .map(|(k, v)| {
                let expression: NifResult<RestrictedExpression> = v.into();
                expression.map(|e| (k, e))
            })
            .collect::<NifResult<RecordItems>>()?)
    }
}

#[derive(NifStruct, Debug)]
#[module = "CedarPolicy.EntityUid"]
pub(crate) struct ExEntityUid {
    type_name: String,
    id: String,
}

impl Into<NifResult<EntityUid>> for ExEntityUid {
    fn into(self) -> NifResult<EntityUid> {
        Ok(EntityUid::from_type_name_and_id(
            EntityTypeName::from_str(&self.type_name).map_err(|errors| {
                Error::Term(Box::new(ExError {
                    source: atoms::entity_uid(),
                    reason: join_errors(errors.iter()),
                }))
            })?,
            EntityId::from_str(&self.id).unwrap(),
        ))
    }
}

#[derive(NifTaggedEnum, Debug)]
pub(crate) enum ExRestrictedExpression {
    Long(i64),
    Bool(bool),
    Ip(String),
    String(String),
    Decimal(String),
    DateTime(String),
    Duration(String),
    Entity(ExEntity),
    Record(Vec<ExRecordItem>),
    Set(Vec<ExRestrictedExpression>),
}

impl Into<NifResult<RestrictedExpression>> for ExRestrictedExpression {
    fn into(self) -> NifResult<RestrictedExpression> {
        match self {
            ExRestrictedExpression::Ip(value) => Ok(RestrictedExpression::new_ip(value)),
            ExRestrictedExpression::Bool(value) => Ok(RestrictedExpression::new_bool(value)),
            ExRestrictedExpression::Long(value) => Ok(RestrictedExpression::new_long(value)),
            ExRestrictedExpression::String(value) => Ok(RestrictedExpression::new_string(value)),
            ExRestrictedExpression::Decimal(value) => Ok(RestrictedExpression::new_decimal(value)),
            ExRestrictedExpression::DateTime(value) => {
                Ok(RestrictedExpression::new_datetime(value))
            }
            ExRestrictedExpression::Duration(value) => {
                Ok(RestrictedExpression::new_duration(value))
            }
            ExRestrictedExpression::Set(value) => Ok(RestrictedExpression::new_set(
                value
                    .into_iter()
                    .map(|v| v.into())
                    .collect::<NifResult<Vec<RestrictedExpression>>>()?,
            )),
            ExRestrictedExpression::Entity(value) => {
                let entity_uid: NifResult<EntityUid> = value.id.into();
                Ok(RestrictedExpression::new_entity_uid(entity_uid?))
            }
            ExRestrictedExpression::Record(r) => {
                let values: NifResult<RecordItems> = ExRecordItems(r).into();
                Ok(RestrictedExpression::new_record(values?).map_err(|error| {
                    Error::Term(Box::new(ExError {
                        source: atoms::restricted_expression(),
                        reason: error.to_string(),
                    }))
                })?)
            }
        }
    }
}

pub(crate) fn join_errors<T>(value: T) -> String
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
