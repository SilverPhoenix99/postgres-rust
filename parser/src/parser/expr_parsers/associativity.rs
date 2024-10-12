#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum Associativity {
    Left(i16),
    Right(i16),
    Non(i16),
}

impl Associativity {
    #[inline(always)]
    pub fn precedence(&self) -> i16 {
        match self {
            Self::Right(prec)
            | Self::Left(prec)
            | Self::Non(prec)=> *prec,
        }
    }

    #[inline(always)]
    pub fn right_precedence(&self) -> i16 {
        match self {
            Self::Right(prec) => *prec,
            Self::Left(prec) | Self::Non(prec) => *prec + 1,
        }
    }

    #[inline(always)]
    pub fn max_precedence(&self) -> i16 {
        self.precedence() - 1
    }
}
