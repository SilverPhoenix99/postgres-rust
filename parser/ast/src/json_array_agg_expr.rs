#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonArrayAggExpr {
    agg: JsonArrayAgg,
    filter: Option<ExprNode>,
    over_clause: Option<OverClause>
}

impl JsonArrayAggExpr {
    pub fn new<T: Into<JsonArrayAgg>>(agg: T) -> Self {
        Self {
            agg: agg.into(),
            filter: None,
            over_clause: None,
        }
    }

    pub fn agg(&self) -> &JsonArrayAgg {
        &self.agg
    }

    pub fn set_filter(&mut self, filter: Option<ExprNode>) -> &mut Self {
        self.filter = filter;
        self
    }

    pub fn with_filter(mut self, filter: ExprNode) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn filter(&self) -> Option<&ExprNode> {
        self.filter.as_ref()
    }

    pub fn set_over_clause(&mut self, over_clause: Option<OverClause>) -> &mut Self {
        self.over_clause = over_clause;
        self
    }

    pub fn with_over(mut self, over_clause: OverClause) -> Self {
        self.over_clause = Some(over_clause);
        self
    }

    pub fn over_clause(&self) -> Option<&OverClause> {
        self.over_clause.as_ref()
    }
}

use crate::ExprNode;
use crate::JsonArrayAgg;
use crate::OverClause;
