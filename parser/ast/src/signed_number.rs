#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum SignedNumber {
    #[from] IntegerConst(i32),
    NumericConst { value: Box<str>, radix: NumberRadix, negative: bool },
}

impl core::ops::Neg for SignedNumber {
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

use crate::UnsignedNumber;
use derive_more::From;
use pg_basics::NumberRadix;
