#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonSerializeExpr {
    expr: JsonValueExpr,
    output: Option<JsonOutput>
}

impl JsonSerializeExpr {
    pub fn new(expr: JsonValueExpr) -> Self {
        Self {
            expr,
            output: None,
        }
    }

    pub fn expr(&self) -> &JsonValueExpr {
        &self.expr
    }

    pub fn set_output(&mut self, output: Option<JsonOutput>) -> &mut Self {
        self.output = output;
        self
    }

    pub fn with_output(mut self, output: JsonOutput) -> Self {
        self.output = Some(output);
        self
    }

    pub fn output(&self) -> Option<&JsonOutput> {
        self.output.as_ref()
    }
}

use crate::JsonOutput;
use crate::JsonValueExpr;
