#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GraphElementPatternKind {
    VertexPattern(GraphElementPattern),
    EdgePatternLeft(GraphElementPattern),
    EdgePatternRight(GraphElementPattern),
    EdgePatternAny(GraphElementPattern),
    ParenExpr {
        sub_expr: Box<GraphElementPattern>,
        where_clause: Option<ExprNode>,
    },
}

use crate::ExprNode;
use crate::GraphElementPattern;
