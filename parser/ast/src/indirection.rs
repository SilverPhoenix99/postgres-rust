#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Indirection {

    /// `.*`
    All,

    /// `.ColLabel`
    Property(Str),

    /// `[expr]`
    Index(ExprNode),

    /// Slice notation:
    /// * `[:]`
    /// * `[ expr : ]`
    /// * `[ : expr ]`
    /// * `[ expr : expr ]`
    Slice(Option<ExprNode>, Option<ExprNode>),
}

use crate::ExprNode;
use pg_basics::Str;
