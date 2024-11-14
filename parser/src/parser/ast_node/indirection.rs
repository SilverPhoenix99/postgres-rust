#[derive(Debug, Clone, PartialEq)]
pub enum Indirection {
    /// `.*`
    All,
    /// `.ColLabel`
    Property(Str),
    /// `[expr]`
    Index(ExprNode),
    /// `[:]`
    FullSlice,
    /// `[ expr : ]`
    SliceFrom(ExprNode),
    /// `[ : expr ]`
    SliceTo(ExprNode),
    /// `[ expr : expr ]`
    Slice(ExprNode, ExprNode),
}

use crate::parser::ast_node::ExprNode;
use postgres_basics::Str;
