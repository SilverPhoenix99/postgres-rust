use bitvec::boxed::BitBox;
use std::borrow::Cow;
use std::num::NonZero;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(Cow<'static, str>),
}

// If limited, the maximum is 10MB == 10,485,760
// see https://www.postgresql.org/docs/current/datatype-character.html
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CharacterType {
    Varchar(Option<NonZero<u32>>),
    Bpchar(Option<NonZero<u32>>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NumericSpec {
    precision: NonZero<u16>,
    scale: i16,
}

impl NumericSpec {

    pub const VALID_SPECIFIED_PRECISION: RangeInclusive<u16> = 1..=1000;
    pub const VALID_SPECIFIED_SCALE: RangeInclusive<i16> = -1000..=1000;

    /// The total amount of digits, before + after the decimal point,
    /// that a numeric value can have, when the precision and scale are **both** unspecified.
    pub const UNSPECIFIED_PRECISION: u32 = 131072 + 16383;

    /// The total amount of digits after the decimal point,
    /// that a numeric value can have, when the precision and scale are **both** unspecified.
    pub const UNSPECIFIED_SCALE: i32 = 16383;

    #[inline]
    pub fn new(precision: NonZero<u16>, scale: Option<i16>) -> Self {


        match scale {
            None => Self::with_precision(precision),
            Some(scale) => {
                debug_assert!(Self::VALID_SPECIFIED_PRECISION.contains(&precision.get()));
                debug_assert!(Self::VALID_SPECIFIED_SCALE.contains(&scale));
                Self { precision, scale }
            },
        }
    }

    #[inline(always)]
    pub fn with_precision(precision: NonZero<u16>) -> Self {
        debug_assert!(Self::VALID_SPECIFIED_PRECISION.contains(&precision.get()));
        Self { precision, scale: 0 }
    }

    /// Total number of digits the value can have, before & after the decimal point.
    /// * If specified, it must be <= 1000;
    /// * If unspecified, the maximum is 131,072 digits before the decimal point, and 16,383 after.
    #[inline(always)]
    pub fn precision(&self) -> u32 {

        if self.precision.get() == u16::MAX {
            // Hack: MAX is an invalid precision, and it's used internally to keep the field compact.
            Self::UNSPECIFIED_PRECISION
        }
        else {
            self.precision.get() as u32
        }
    }

    /// Number of fractional decimal digits.
    /// * If specified, it must be >= -1000 and <= 1000;
    /// * If `precision` is specified, and `scale` is unspecified, it's defaulted to 0.
    /// * If unspecified, the maximum is 16,383 digits after the decimal point.
    #[inline(always)]
    pub fn scale(&self) -> i32 {

        if self.scale == i16::MAX {
            // Hack: MAX is an invalid scale, and it's used internally to keep the field compact.
            Self::UNSPECIFIED_SCALE
        }
        else {
            self.scale as i32
        }
    }
}

impl Default for NumericSpec {

    #[inline(always)]
    fn default() -> Self {

        // Hack: using the MAX values to represent Self::UNSPECIFIED_* values,
        //       because the latter don't fit in 16 bit ints.

        Self {
            precision: unsafe {
                // SAFETY: MAX is never 0
                NonZero::new_unchecked(u16::MAX)
            },
            scale: i16::MAX
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NumericType {
    Bool,
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    Numeric(NumericSpec),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SystemType {
    Character(CharacterType),
    Numeric(NumericType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    StringLiteral(String),
    BitStringLiteral(BitBox),
    IntegerLiteral(i32),
    FloatLiteral(f64),
    NumericLiteral(String), // TODO: Replace with some kind of BigDecimal
    BooleanLiteral(bool),
    NullLiteral,
}
