#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GraphElementPatternKind {
    VertexPattern(GraphElementPattern),
    EdgePatternLeft(GraphElementPattern),
    EdgePatternRight(GraphElementPattern),
    EdgePatternAny(GraphElementPattern),
    ParenExpr {
        sub_expr: Vec<GraphElementPatternKind>,
        where_clause: Option<ExprNode>,
        quantifier: Option<RangeInclusive<NonNegative>>,
    },
}

impl GraphElementPatternKind {
    
    pub fn set_quantifier(&mut self, quantifier: Option<RangeInclusive<NonNegative>>) -> &mut Self {
        
        match self {
            Self::VertexPattern(pattern) => { pattern.set_quantifier(quantifier); }
            Self::EdgePatternLeft(pattern) => { pattern.set_quantifier(quantifier); }
            Self::EdgePatternRight(pattern) => { pattern.set_quantifier(quantifier); }
            Self::EdgePatternAny(pattern) => { pattern.set_quantifier(quantifier); }
            Self::ParenExpr { quantifier: q, .. } => { *q = quantifier; }
        }

        self
    }
}

use crate::ExprNode;
use crate::GraphElementPattern;
use core::ops::RangeInclusive;
use pg_basics::NonNegative;
