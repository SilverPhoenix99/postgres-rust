/// Alias: `RangeFunction`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionTableRef {
    function: FuncExprWindowless,
    alias: Option<FuncAlias>,
    ordinality: bool,
    lateral: bool,
}

impl FunctionTableRef {
    pub fn new<F: Into<FuncExprWindowless>>(function: F) -> Self {
        Self {
            function: function.into(),
            alias: None,
            ordinality: false,
            lateral: false,
        }
    }

    pub fn function(&self) -> &FuncExprWindowless {
        &self.function
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

impl From<FuncExprWindowless> for FunctionTableRef {
    fn from(function: FuncExprWindowless) -> Self {
        Self::new(function)
    }
}

impl From<SqlFunction> for FunctionTableRef {
    fn from(value: SqlFunction) -> Self {
        Self::new(value)
    }
}

impl From<FuncCall> for FunctionTableRef {
    fn from(value: FuncCall) -> Self {
        Self::new(value)
    }
}

use crate::FuncExprWindowless;
use crate::{FuncAlias, FuncCall, SqlFunction};
