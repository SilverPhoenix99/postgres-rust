#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonTable {
    context: JsonValueExpr,
    path_spec: JsonTablePathSpec,
    columns: Vec<JsonTableColumnDefinition>,
    passing: Option<Vec<JsonArgument>>,
    on_error: Option<JsonBehavior>,
}

impl JsonTable {
    pub fn new(
        context: JsonValueExpr,
        path_spec: JsonTablePathSpec,
        columns: Vec<JsonTableColumnDefinition>,
    ) -> Self {
        Self {
            context,
            path_spec,
            columns,
            passing: None,
            on_error: None,
        }
    }

    pub fn context(&self) -> &JsonValueExpr {
        &self.context
    }

    pub fn path_spec(&self) -> &JsonTablePathSpec {
        &self.path_spec
    }

    pub fn columns(&self) -> &[JsonTableColumnDefinition] {
        &self.columns
    }

    pub fn set_passing(&mut self, passing: Option<Vec<JsonArgument>>) -> &mut Self {
        self.passing = passing;
        self
    }

    pub fn with_passing(mut self, passing: Vec<JsonArgument>) -> Self {
        self.passing = Some(passing);
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

use crate::JsonArgument;
use crate::JsonBehavior;
use crate::JsonTableColumnDefinition;
use crate::JsonTablePathSpec;
use crate::JsonValueExpr;
