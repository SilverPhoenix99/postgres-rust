#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortBy {
    expr: ExprNode,
    direction: Option<SortDirection>,
    nulls: Option<SortNulls>
}

impl SortBy {
    pub fn new(
        expr: ExprNode,
        direction: Option<SortDirection>,
        nulls: Option<SortNulls>
    ) -> Self {
        Self { expr, direction, nulls }
    }

    pub fn expr(&self) -> &ExprNode {
        &self.expr
    }

    pub fn direction(&self) -> Option<&SortDirection> {
        self.direction.as_ref()
    }

    pub fn nulls(&self) -> Option<SortNulls> {
        self.nulls
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
    Using(QualifiedOperator)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortNulls {
    NullsFirst,
    NullsLast,
}

use crate::ExprNode;
use pg_sink_ast::QualifiedOperator;
