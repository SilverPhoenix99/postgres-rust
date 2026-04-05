#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum JoinQual {
    #[default]
    Natural,
    Using(Vec<Str>),
    On(Box<ExprNode>),
}

use crate::ExprNode;
use pg_basics::Str;
