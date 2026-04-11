#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoinKind {
    Inner(Option<JoinQual>),
    Left(JoinQual),
    Full(JoinQual),
    Right(JoinQual),
}

impl Default for JoinKind {
    /// `NATURAL INNER JOIN`
    fn default() -> Self {
        Self::Inner(Some(Default::default()))
    }
}

impl JoinKind {
    pub fn cross_join() -> Self {
        Self::Inner(None)
    }

    pub fn is_cross_join(&self) -> bool {
        matches!(self, Self::Inner(None))
    }
}

use crate::JoinQual;
