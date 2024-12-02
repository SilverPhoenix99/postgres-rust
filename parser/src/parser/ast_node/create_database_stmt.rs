#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateDatabaseStmt {
    name: Str,
    options: Vec<CreatedbOption>,
}

impl CreateDatabaseStmt {
    pub fn new(name: Str, options: Vec<CreatedbOption>) -> Self {
        Self { name, options }
    }

    pub fn name(&self) -> &Str {
        &self.name
    }

    pub fn options(&self) -> &Vec<CreatedbOption> {
        &self.options
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreatedbOption {
    kind: CreatedbOptionKind,
    value: CreatedbOptionValue,
}

impl CreatedbOption {
    pub fn new(kind: CreatedbOptionKind, value: CreatedbOptionValue) -> Self {
        CreatedbOption { kind, value }
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

use crate::parser::ast_node::SignedNumber;
use postgres_basics::Str;
