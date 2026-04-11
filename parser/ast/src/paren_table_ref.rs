#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenTableRef {
    table_ref: TableRef,
    alias: Alias,
}

impl ParenTableRef {
    pub fn new<T: Into<TableRef>, A: Into<Alias>>(table_ref: T, alias: A) -> Self {
        Self {
            table_ref: table_ref.into(),
            alias: alias.into(),
        }
    }

    pub fn table_ref(&self) -> &TableRef {
        &self.table_ref
    }

    pub fn alias(&self) -> &Alias {
        &self.alias
    }
}

use crate::Alias;
use crate::TableRef;
