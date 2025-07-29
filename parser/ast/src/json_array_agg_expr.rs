#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonArrayAggExpr {
    agg: JsonArrayAgg,
    filter: Option<ExprNode>,
    over_clause: Option<OverClause>
}

impl JsonArrayAggExpr {
    pub fn new(agg: JsonArrayAgg, filter: Option<ExprNode>, over_clause: Option<OverClause>) -> Self {
        Self { agg, filter, over_clause }
    }

    pub fn agg(&self) -> &JsonArrayAgg {
        &self.agg
    }

    pub fn filter(&self) -> Option<&ExprNode> {
        self.filter.as_ref()
    }

    pub fn over_clause(&self) -> Option<&OverClause> {
        self.over_clause.as_ref()
    }
}

use crate::ExprNode;
use crate::JsonArrayAgg;
use crate::OverClause;
