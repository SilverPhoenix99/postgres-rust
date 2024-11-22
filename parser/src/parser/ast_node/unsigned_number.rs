#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnsignedNumber {
    IntegerConst(NonNegative),
    NumericConst { value: Box<str>, radix: NumberRadix },
}

use crate::NumberRadix;
use postgres_basics::NonNegative;
