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

impl_from!(NonNegative for UnsignedNumber::IntegerConst);

use pg_basics::impl_from;
use pg_basics::NonNegative;
use pg_basics::NumberRadix;
