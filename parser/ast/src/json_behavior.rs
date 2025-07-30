#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct JsonBehaviorClause {
    on_error: Option<JsonBehavior>,
    on_empty: Option<JsonBehavior>,
}

impl JsonBehaviorClause {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_on_error(mut self, behavior: JsonBehavior) -> Self {
        self.on_error = Some(behavior);
        self
    }

    pub fn on_error(&self) -> Option<&JsonBehavior> {
        self.on_error.as_ref()
    }

    pub fn with_on_empty(mut self, behavior: JsonBehavior) -> Self {
        self.on_empty = Some(behavior);
        self
    }

    pub fn on_empty(&self) -> Option<&JsonBehavior> {
        self.on_empty.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonBehavior {
    Null,
    Error,
    True,
    False,
    Unknown,
    EmptyArray,
    EmptyObject,
    Default(ExprNode),
}

use crate::ExprNode;
