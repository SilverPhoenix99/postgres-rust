#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum SignedNumber {
    #[from] IntegerConst(i32),
    #[from] NumericConst(Number),
}

impl core::ops::Neg for SignedNumber {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::IntegerConst(int) => Self::IntegerConst(-int),
            Self::NumericConst(number) => Self::NumericConst(-number),
        }
    }
}

impl From<UnsignedNumber> for SignedNumber {
    fn from(value: UnsignedNumber) -> Self {
        match value {
            // SAFETY: `int` is originally parsed by `i32::from_str_radix()`, so `0 <= int <= i32::MAX`
            UnsignedNumber::IntegerConst(int) => Self::IntegerConst(int.into()),
            UnsignedNumber::NumericConst { value, radix } => Self::NumericConst(
                Number::new(value, radix)
            ),
        }
    }
}

use crate::Number;
use derive_more::From;
use pg_basics::UnsignedNumber;
