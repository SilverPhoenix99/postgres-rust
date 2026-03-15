/// Alias: `RangeFunction`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowsTableRef {
    functions: Vec<RangeFunction>,
    alias: Option<FuncAlias>,
    ordinality: bool,
}

impl RowsTableRef {
    pub fn new(functions: Vec<RangeFunction>) -> Self {
        Self {
            functions,
            alias: None,
            ordinality: false,
        }
    }

    pub fn functions(&self) -> &[RangeFunction] {
        &self.functions
    }

    pub fn set_alias(&mut self, alias: Option<FuncAlias>) -> &mut Self {
        self.alias = alias;
        self
    }

    pub fn with_alias(mut self, alias: FuncAlias) -> Self {
        self.alias = Some(alias);
        self
    }

    pub fn alias(&self) -> Option<&FuncAlias> {
        self.alias.as_ref()
    }

    pub fn set_ordinality(&mut self, ordinality: bool) -> &mut Self {
        self.ordinality = ordinality;
        self
    }

    pub fn with_ordinality(mut self, ordinality: bool) -> Self {
        self.ordinality = ordinality;
        self
    }

    pub fn ordinality(&self) -> bool {
        self.ordinality
    }
}

use crate::FuncAlias;
use crate::RangeFunction;
