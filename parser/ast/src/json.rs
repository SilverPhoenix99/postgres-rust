#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonFunc {
    value: JsonValueExpr,
    unique: bool,
}

impl JsonFunc {
    pub fn new<T: Into<JsonValueExpr>>(value: T) -> Self {
        Self {
            value: value.into(),
            unique: false,
        }
    }

    pub fn value(&self) -> &JsonValueExpr {
        &self.value
    }

    pub fn set_unique(&mut self, unique: bool) -> &mut Self {
        self.unique = unique;
        self
    }
    
    pub fn with_unique(mut self, unique: bool) -> Self {
        self.unique = unique;
        self
    }
    
    pub fn is_unique(&self) -> bool {
        self.unique
    }
}

use crate::JsonValueExpr;
