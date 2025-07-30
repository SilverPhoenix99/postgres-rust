#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonValueFunc {
    context_item: JsonValueExpr,
    path_spec: ExprNode,
    passing: Option<Vec<JsonArgument>>,
    output: Option<JsonOutput>,
    behavior: JsonBehaviorClause,
}

impl JsonValueFunc {
    pub fn new(context_item: JsonValueExpr, path_spec: ExprNode) -> Self {
        Self {
            context_item,
            path_spec,
            passing: None,
            output: None,
            behavior: JsonBehaviorClause::default(),
        }
    }

    pub fn context_item(&self) -> &JsonValueExpr {
        &self.context_item
    }

    pub fn path_spec(&self) -> &ExprNode {
        &self.path_spec
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
    
    pub fn set_behavior(&mut self, behavior: JsonBehaviorClause) -> &mut Self {
        self.behavior = behavior;
        self
    }
    
    pub fn with_behavior(mut self, behavior: JsonBehaviorClause) -> Self {
        self.behavior = behavior;
        self
    }

    pub fn behavior(&self) -> &JsonBehaviorClause {
        &self.behavior
    }
}

use crate::ExprNode;
use crate::JsonArgument;
use crate::JsonBehaviorClause;
use crate::JsonOutput;
use crate::JsonValueExpr;
