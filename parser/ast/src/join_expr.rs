#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JoinExpr {
    kind: JoinKind,
    left: TableRef,
    right: TableRef,
    alias: Option<Alias>,
}

impl JoinExpr {
    pub fn new<L: Into<TableRef>, R: Into<TableRef>>(kind: JoinKind, left: L, right: R) -> Self {
        Self {
            kind,
            left: left.into(),
            right: right.into(),
            alias: None
        }
    }

    pub fn kind(&self) -> &JoinKind {
        &self.kind
    }

    pub fn left(&self) -> &TableRef {
        &self.left
    }

    pub fn right(&self) -> &TableRef {
        &self.right
    }

    pub fn set_alias(&mut self, alias: Option<Alias>) -> &mut Self {
        self.alias = alias;
        self
    }

    pub fn with_alias<T: Into<Alias>>(mut self, alias: T) -> Self {
        self.alias = Some(alias.into());
        self
    }

    pub fn alias(&self) -> Option<&Alias> {
        self.alias.as_ref()
    }
}

use crate::Alias;
use crate::JoinKind;
use crate::TableRef;
