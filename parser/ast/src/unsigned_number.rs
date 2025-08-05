#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum UnsignedNumber {
    #[from] IntegerConst(NonNegative),
    NumericConst { value: Box<str>, radix: NumberRadix },
}

impl Default for UnsignedNumber {
    fn default() -> Self {
        Self::IntegerConst(Default::default())
    }
}

use derive_more::From;
use pg_basics::NonNegative;
use pg_basics::NumberRadix;
