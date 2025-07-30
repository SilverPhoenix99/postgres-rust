pub type JsonArgument = (Str, JsonValueExpr);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonExistsExpr {
    context_item: JsonValueExpr,
    path_expression: ExprNode,
    passing: Option<Vec<JsonArgument>>,
    on_error: Option<JsonBehavior>,
}

impl JsonExistsExpr {
    pub fn new(
        context_item: JsonValueExpr,
        path_expression: ExprNode,
        passing: Option<Vec<JsonArgument>>,
        on_error: Option<JsonBehavior>,
    ) -> Self {
        Self {
            context_item,
            path_expression,
            passing,
            on_error,
        }
    }

    pub fn context_item(&self) -> &JsonValueExpr {
        &self.context_item
    }

    pub fn path_expression(&self) -> &ExprNode {
        &self.path_expression
    }

    pub fn passing(&self) -> Option<&[JsonArgument]> {
        self.passing.as_deref()
    }

    pub fn on_error(&self) -> Option<&JsonBehavior> {
        self.on_error.as_ref()
    }
}

use crate::ExprNode;
use crate::JsonBehavior;
use crate::JsonValueExpr;
use pg_basics::Str;
