#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonArrayConstructor {
    expressions: Vec<JsonValueExpr>,
    absent_on_null: bool,
    output: Option<JsonOutput>,
}

impl JsonArrayConstructor {

    pub fn new(expressions: Vec<JsonValueExpr>) -> Self {
        Self {
            expressions,
            output: None,
            absent_on_null: true,
        }
    }

    pub fn expressions(&self) -> &[JsonValueExpr] {
        self.expressions.as_ref()
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

    pub fn set_absent_on_null(&mut self, absent_on_null: bool) -> &mut Self {
        self.absent_on_null = absent_on_null;
        self
    }

    pub fn with_absent_on_null(mut self, absent_on_null: bool) -> Self {
        self.absent_on_null = absent_on_null;
        self
    }

    pub fn absent_on_null(&self) -> bool {
        self.absent_on_null
    }
}

use crate::JsonOutput;
use crate::JsonValueExpr;
