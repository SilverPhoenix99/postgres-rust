#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OneOrBoth<L, R = L> {
    Left(L),
    Right(R),
    Both(L, R),
}

impl <L, R> OneOrBoth<L, R> {
    pub fn left(&self) -> Option<&L> {
        match self {
            Self::Left(left) => Some(left),
            Self::Both(left, _) => Some(left),
            _ => None,
        }
    }

    pub fn right(&self) -> Option<&R> {
        match self {
            Self::Right(right) => Some(right),
            Self::Both(_, right) => Some(right),
            _ => None,
        }
    }
}
