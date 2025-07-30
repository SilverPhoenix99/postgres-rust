#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NamedValue {
    name: Option<Str>,
    value: ExprNode,
}

impl NamedValue {
    pub fn new(name: Option<Str>, value: ExprNode) -> Self {
        Self { name, value }
    }

    pub fn unnamed(value: ExprNode) -> Self {
        Self {
            name: None,
            value,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn value(&self) -> &ExprNode {
        &self.value
    }
}

impl From<NamedValue> for (Option<Str>, ExprNode) {
    fn from(value: NamedValue) -> Self {
        (value.name, value.value)
    }
}

use crate::ExprNode;
use pg_basics::Str;
