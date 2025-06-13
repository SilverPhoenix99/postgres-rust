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

    pub fn as_ref(&self) -> OneOrBoth<&L, &R> {
        match *self {
            Self::Left(ref left) => OneOrBoth::Left(left),
            Self::Right(ref right) => OneOrBoth::Right(right),
            Self::Both(ref left, ref right) => OneOrBoth::Both(left, right),
        }
    }
}

impl<L, R> OneOrBoth<L, R>
where
    L: Deref,
    R: Deref,
{
    pub fn as_deref(&self) -> OneOrBoth<&L::Target, &R::Target> {
        match *self {
            Self::Left(ref left) => OneOrBoth::Left(left.deref()),
            Self::Right(ref right) => OneOrBoth::Right(right.deref()),
            Self::Both(ref left, ref right) => OneOrBoth::Both(left.deref(), right.deref()),
        }
    }
}

impl<L, R> OneOrBoth<L, R>
where
    L: Deref
{
    pub fn as_left_deref(&self) -> OneOrBoth<&L::Target, &R> {
        match *self {
            Self::Left(ref left) => OneOrBoth::Left(left.deref()),
            Self::Right(ref right) => OneOrBoth::Right(right),
            Self::Both(ref left, ref right) => OneOrBoth::Both(left.deref(), right),
        }
    }
}

impl<L, R> OneOrBoth<L, R>
where
    R: Deref,
{
    pub fn as_right_deref(&self) -> OneOrBoth<&L, &R::Target> {
        match *self {
            Self::Left(ref left) => OneOrBoth::Left(left),
            Self::Right(ref right) => OneOrBoth::Right(right.deref()),
            Self::Both(ref left, ref right) => OneOrBoth::Both(left, right.deref()),
        }
    }
}

use core::ops::Deref;
