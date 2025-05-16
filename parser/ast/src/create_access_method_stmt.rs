/// Alias: `CreateAmStmt`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateAccessMethodStmt {
    name: Str,
    kind: AccessMethodKind,
    handler: QualifiedName,
}

impl CreateAccessMethodStmt {
    pub fn new<T: Into<Str>>(name: T, kind: AccessMethodKind, handler: QualifiedName) -> Self {
        Self {
            name: name.into(),
            kind,
            handler
        }
    }

    pub fn name(&self) -> &Str {
        &self.name
    }

    pub fn kind(&self) -> AccessMethodKind {
        self.kind
    }

    pub fn handler(&self) -> &QualifiedName {
        &self.handler
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccessMethodKind {
    Index,
    Table
}

use pg_basics::QualifiedName;
use pg_basics::Str;
