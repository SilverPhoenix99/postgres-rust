pub type JsonArgument = (Str, JsonValueExpr);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonExistsExpr {
    context_item: JsonValueExpr,
    path_spec: ExprNode,
    passing: Option<Vec<JsonArgument>>,
    on_error: Option<JsonBehavior>,
}

impl JsonExistsExpr {
    pub fn new(
        context_item: JsonValueExpr,
        path_spec: ExprNode
    ) -> Self {
        Self {
            context_item,
            path_spec,
            passing: None,
            on_error: None,
        }
    }

    pub fn context_item(&self) -> &JsonValueExpr {
        &self.context_item
    }

    pub fn path_spec(&self) -> &ExprNode {
        &self.path_spec
    }

    pub fn set_passing(&mut self, passing: Option<Vec<JsonArgument>>) -> &mut Self {

        self.passing = passing.and_then(|passing|
            if passing.is_empty() { None }
            else { Some(passing) }
        );

        self
    }

    pub fn with_passing(mut self, passing: Vec<JsonArgument>) -> Self {
        self.passing = if passing.is_empty() { None } else { Some(passing) };
        self
    }

    pub fn passing(&self) -> Option<&[JsonArgument]> {
        self.passing.as_deref()
    }

    pub fn set_on_error(&mut self, on_error: Option<JsonBehavior>) -> &mut Self {
        self.on_error = on_error;
        self
    }

    pub fn with_on_error(mut self, on_error: JsonBehavior) -> Self {
        self.on_error = Some(on_error);
        self
    }

    pub fn on_error(&self) -> Option<&JsonBehavior> {
        self.on_error.as_ref()
    }
}

use crate::ExprNode;
use crate::JsonBehavior;
use crate::JsonValueExpr;
use pg_basics::Str;
