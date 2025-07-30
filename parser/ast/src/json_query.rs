#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonQueryExpr {
    context_item: JsonValueExpr,
    path_spec: ExprNode,
    passing: Option<Vec<JsonArgument>>,
    output: Option<JsonOutput>,
    wrapper: Option<JsonWrapperBehavior>,
    quotes: Option<JsonQuotes>,
    behavior: Option<JsonBehaviorClause>
}

impl JsonQueryExpr {
    pub fn new(
        context_item: JsonValueExpr,
        path_spec: ExprNode,
    ) -> Self {
        Self {
            context_item,
            path_spec,
            passing: None,
            output: None,
            wrapper: None,
            quotes: None,
            behavior: None,
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

    pub fn set_wrapper(&mut self, wrapper: Option<JsonWrapperBehavior>) -> &mut Self {
        self.wrapper = wrapper;
        self
    }

    pub fn with_wrapper(mut self, wrapper: JsonWrapperBehavior) -> Self {
        self.wrapper = Some(wrapper);
        self
    }

    pub fn wrapper(&self) -> Option<JsonWrapperBehavior> {
        self.wrapper
    }

    pub fn set_quotes(&mut self, quotes: Option<JsonQuotes>) -> &mut Self {
        self.quotes = quotes;
        self
    }

    pub fn with_quotes(mut self, quotes: JsonQuotes) -> Self {
        self.quotes = Some(quotes);
        self
    }

    pub fn quotes(&self) -> Option<JsonQuotes> {
        self.quotes
    }

    pub fn set_behavior(&mut self, behavior: Option<JsonBehaviorClause>) -> &mut Self {
        self.behavior = behavior;
        self
    }

    pub fn with_behavior(mut self, behavior: JsonBehaviorClause) -> Self {
        self.behavior = Some(behavior);
        self
    }

    pub fn behavior(&self) -> Option<&JsonBehaviorClause> {
        self.behavior.as_ref()
    }
}

use crate::ExprNode;
use crate::JsonArgument;
use crate::JsonBehaviorClause;
use crate::JsonOutput;
use crate::JsonQuotes;
use crate::JsonValueExpr;
use crate::JsonWrapperBehavior;
