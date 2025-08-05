#[derive(Debug, Clone, Eq, PartialEq, Into)]
#[into((Option<Str>, ExprNode))]
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

use crate::ExprNode;
use derive_more::Into;
use pg_basics::Str;
