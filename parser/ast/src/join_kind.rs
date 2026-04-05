#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoinKind {
    Inner(Option<JoinQual>),
    Left(JoinQual),
    Full(JoinQual),
    Right(JoinQual),
}

use crate::JoinQual;
