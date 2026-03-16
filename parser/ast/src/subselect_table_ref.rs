#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubselectTableRef {
    select: SelectStmt,
    alias: Option<Alias>,
    lateral: bool,
}

impl SubselectTableRef {
    pub fn new(select: SelectStmt) -> Self {
        Self {
            select,
            alias: None,
            lateral: false,
        }
    }

    pub fn select(&self) -> &SelectStmt {
        &self.select
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

    pub fn set_lateral(&mut self, lateral: bool) -> &mut Self {
        self.lateral = lateral;
        self
    }

    pub fn with_lateral(mut self, lateral: bool) -> Self {
        self.lateral = lateral;
        self
    }

    pub fn lateral(&self) -> bool {
        self.lateral
    }
}

use crate::Alias;
use crate::SelectStmt;
