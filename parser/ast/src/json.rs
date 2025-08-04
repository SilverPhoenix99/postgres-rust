#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonFunc {
    value: JsonValueExpr,
    unique: bool,
}

impl JsonFunc {
    pub fn new(value: JsonValueExpr, unique: bool) -> Self {
        Self { value, unique }
    }

    pub fn value(&self) -> &JsonValueExpr {
        &self.value
    }

    pub fn is_unique(&self) -> bool {
        self.unique
    }
}

use crate::JsonValueExpr;
