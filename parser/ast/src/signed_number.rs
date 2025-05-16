#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SignedNumber {
    IntegerConst(i32),
    NumericConst { value: Box<str>, radix: NumberRadix, negative: bool },
}

impl std::ops::Neg for SignedNumber {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            SignedNumber::IntegerConst(int) => SignedNumber::IntegerConst(-int),
            SignedNumber::NumericConst { value, radix, negative } => {
                SignedNumber::NumericConst {
                    value,
                    radix,
                    negative: !negative,
                }
            }
        }
    }
}

impl From<UnsignedNumber> for SignedNumber {
    fn from(value: UnsignedNumber) -> Self {
        match value {
            UnsignedNumber::IntegerConst(int) => Self::IntegerConst(int.into()),
            UnsignedNumber::NumericConst { value, radix } => Self::NumericConst { value, radix, negative: false },
        }
    }
}

impl_from!(i32 for SignedNumber::IntegerConst);

use crate::UnsignedNumber;
use pg_basics::impl_from;
use pg_basics::NumberRadix;
