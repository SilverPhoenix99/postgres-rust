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

impl GraphElementPatternKind {

    pub fn pattern(&self) -> &GraphElementPattern {
        match self {
            Self::VertexPattern(pattern) => pattern,
            Self::EdgePatternLeft(pattern) => pattern,
            Self::EdgePatternRight(pattern) => pattern,
            Self::EdgePatternAny(pattern) => pattern,
            Self::ParenExpr { sub_expr, .. } => sub_expr,
        }
    }

    pub fn pattern_mut(&mut self) -> &mut GraphElementPattern {
        match self {
            Self::VertexPattern(pattern) => pattern,
            Self::EdgePatternLeft(pattern) => pattern,
            Self::EdgePatternRight(pattern) => pattern,
            Self::EdgePatternAny(pattern) => pattern,
            Self::ParenExpr { sub_expr, .. } => sub_expr,
        }
    }
}

use crate::ExprNode;
use crate::GraphElementPattern;
