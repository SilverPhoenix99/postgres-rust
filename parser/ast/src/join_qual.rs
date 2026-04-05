#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoinQual {
    Natural,
    Using(Vec<Str>),
    On(Box<ExprNode>),
}

use crate::ExprNode;
use pg_basics::Str;
