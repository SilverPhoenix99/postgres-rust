#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterDatabaseStmt {
    name: Str,
    options: Vec<AlterdbOption>
}

impl AlterDatabaseStmt {
    pub fn new<T: Into<Str>>(name: T, options: Vec<AlterdbOption>) -> Self {
        Self {
            name: name.into(),
            options
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn options(&self) -> &[AlterdbOption] {
        &self.options
    }
}

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

// See [`AlterDatabase()`](https://github.com/postgres/postgres/blob/75818b3afbf850d600e0fcd1a3b03199077063f8/src/backend/commands/dbcommands.c#L2363-L2396)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterdbOptionKind {
    AllowConnections,
    ConnectionLimit,
    IsTemplate,
    Tablespace,
    Unknown(Box<str>),
}

use crate::CreatedbOptionValue;
use pg_basics::Str;
