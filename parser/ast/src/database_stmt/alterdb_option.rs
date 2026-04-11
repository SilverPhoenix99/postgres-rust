#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterdbOption {
    kind: AlterdbOptionKind,
    value: CreatedbOptionValue,
}

impl AlterdbOption {
    pub fn new<V: Into<CreatedbOptionValue>>(kind: AlterdbOptionKind, value: V) -> AlterdbOption {
        Self {
            kind,
            value: value.into(),
        }
    }

    pub fn kind(&self) -> &AlterdbOptionKind {
        &self.kind
    }

    /// `ALTER DATABASE` allows the same types of values as `CREATE DATABASE` for its options.
    pub fn value(&self) -> &CreatedbOptionValue {
        &self.value
    }
}

use crate::AlterdbOptionKind;
use crate::CreatedbOptionValue;
