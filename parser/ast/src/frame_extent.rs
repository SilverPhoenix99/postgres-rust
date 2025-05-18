#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameExtent {
    Unbounded { end: Option<PrecedingEnd> },
    CurrentRow { end: Option<CurrentRowEnd> },
    Preceding { 
        start: ExprNode,
        end: Option<PrecedingEnd>
    },
    Following {
        start: ExprNode,
        end: FollowingEnd
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrecedingEnd {
    Unbounded,
    CurrentRow,
    Preceding(ExprNode),
    Following(ExprNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrentRowEnd {
    Unbounded,
    CurrentRow,
    Following(ExprNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FollowingEnd {
    Unbounded,
    Following(ExprNode),
}

use crate::ExprNode;
