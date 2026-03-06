#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreatedbOption {
    kind: CreatedbOptionKind,
    value: CreatedbOptionValue,
}

impl CreatedbOption {
    pub fn new<V: Into<CreatedbOptionValue>>(kind: CreatedbOptionKind, value: V) -> Self {
        CreatedbOption {
            kind,
            value: value.into(),
        }
    }

    pub fn kind(&self) -> &CreatedbOptionKind {
        &self.kind
    }

    pub fn value(&self) -> &CreatedbOptionValue {
        &self.value
    }
}

use crate::CreatedbOptionKind;
use crate::CreatedbOptionValue;
