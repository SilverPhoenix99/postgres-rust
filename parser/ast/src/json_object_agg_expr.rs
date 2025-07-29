#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonObjectAggExpr {
    agg: JsonObjectAgg,
    filter: Option<ExprNode>,
    over_clause: Option<OverClause>
}

impl JsonObjectAggExpr {
    pub fn new(agg: JsonObjectAgg, filter: Option<ExprNode>, over_clause: Option<OverClause>) -> Self {
        Self { agg, filter, over_clause }
    }

    pub fn agg(&self) -> &JsonObjectAgg {
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
use crate::JsonObjectAgg;
use crate::OverClause;
