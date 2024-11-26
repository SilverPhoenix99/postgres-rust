#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnsignedNumber {
    IntegerConst(NonNegative),
    NumericConst { value: Box<str>, radix: NumberRadix },
}

impl Default for UnsignedNumber {
    fn default() -> Self {
        Self::IntegerConst(Default::default())
    }
}

use crate::NumberRadix;
use postgres_basics::NonNegative;
