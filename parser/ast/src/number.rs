#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    value: Box<str>,
    radix: NumberRadix,
    negative: bool,
}

impl Number {
    pub fn new(value: Box<str>, radix: NumberRadix) -> Self {
        Self {
            value,
            radix,
            negative: false
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn radix(&self) -> NumberRadix {
        self.radix
    }

    pub fn is_negative(&self) -> bool {
        self.negative
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            negative: !self.negative,
            ..self
        }
    }
}

use core::ops::Neg;
use pg_basics::NumberRadix;
