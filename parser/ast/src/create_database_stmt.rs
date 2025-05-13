#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateDatabaseStmt {
    name: Str,
    options: Vec<CreatedbOption>,
}

impl CreateDatabaseStmt {
    pub fn new<T: Into<Str>>(name: T, options: Vec<CreatedbOption>) -> Self {
        Self {
            name: name.into(),
            options
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn options(&self) -> &[CreatedbOption] {
        &self.options
    }
}

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

// See [`createdb()`](https://github.com/postgres/postgres/blob/75818b3afbf850d600e0fcd1a3b03199077063f8/src/backend/commands/dbcommands.c#L744-L881)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CreatedbOptionKind {
    AllowConnections,
    BuiltinLocale,
    CollationVersion,
    ConnectionLimit,
    Encoding,
    IcuLocale,
    IcuRules,
    IsTemplate,
    LcCollate,
    LcCtype,
    Locale,
    LocaleProvider,
    /// Undocumented and deprecated.
    ///
    /// [`Tablespace`](crate::parser::ast_node::CreatedbOptionKind::Tablespace) should be used instead.
    Location,
    Owner,
    Tablespace,
    Template,
    Oid,
    Strategy,
    Unknown(Box<str>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CreatedbOptionValue {
    Default,
    Boolean(bool),
    Number(SignedNumber),
    String(Str),
}

impl_from!(SignedNumber for CreatedbOptionValue::Number);
impl_from!(Str for CreatedbOptionValue::String);
impl_from!(String for CreatedbOptionValue::String);
impl_from!(bool for CreatedbOptionValue::Boolean);
impl_from!(i32 for CreatedbOptionValue::Number);

impl From<&'static str> for CreatedbOptionValue {
    fn from(value: &'static str) -> Self {
        Self::String(value.into())
    }
}

impl From<Box<str>> for CreatedbOptionValue {
    fn from(value: Box<str>) -> Self {
        Self::String(value.into())
    }
}

impl From<VarValue> for CreatedbOptionValue {
    fn from(value: VarValue) -> Self {
        match value {
            VarValue::Boolean(value) => Self::Boolean(value),
            VarValue::String(value) => Self::String(value),
            VarValue::Number(value) => Self::Number(value),
        }
    }
}

use crate::SignedNumber;
use crate::VarValue;
use postgres_basics::impl_from;
use postgres_basics::Str;
