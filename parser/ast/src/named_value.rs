#[derive(Debug, Clone, Eq, PartialEq, Into)]
#[into((Option<Str>, ExprNode))]
pub struct NamedValue {
    name: Option<Str>,
    value: ExprNode,
}

impl NamedValue {
    pub fn new(value: ExprNode) -> Self {
        Self {
            name: None,
            value
        }
    }

    pub fn set_name(&mut self, name: Option<Str>) -> &mut Self {
        self.name = name;
        self
    }

    pub fn with_name<T: Into<Str>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
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
