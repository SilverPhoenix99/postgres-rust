use std::num::NonZero;
use std::ops::RangeInclusive;
use NewNumericSpecError::{PrecisionOutOfRange, ScaleOutOfRange};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NewNumericSpecError {
    PrecisionOutOfRange(u16),
    ScaleOutOfRange(i16),
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
    pub fn new(precision: NonZero<u16>, scale: i16) -> Result<Self, NewNumericSpecError> {

        if !Self::VALID_SPECIFIED_PRECISION.contains(&precision.get()) {
            return Err(PrecisionOutOfRange(precision.get()))
        }

        if !Self::VALID_SPECIFIED_SCALE.contains(&scale) {
            return Err(ScaleOutOfRange(scale))
        }

        Ok(Self { precision, scale })
    }

    #[inline(always)]
    pub fn with_precision(precision: NonZero<u16>) -> Result<Self, NewNumericSpecError> {
        Self::new(precision, 0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        unsafe {
            assert_eq!(Err(PrecisionOutOfRange(1001)), NumericSpec::new(NonZero::new_unchecked(1001), 10));
            assert_eq!(Err(ScaleOutOfRange(-1001)), NumericSpec::new(NonZero::new_unchecked(1), -1001));
            assert_eq!(Err(ScaleOutOfRange(1001)), NumericSpec::new(NonZero::new_unchecked(1), 1001));

            let spec = NumericSpec::new(NonZero::new_unchecked(1000), 10).unwrap();
            assert_eq!(1000, spec.precision());
            assert_eq!(10, spec.scale());
        }
    }

    #[test]
    fn test_with_precision() {
        unsafe {
            assert_eq!(Err(PrecisionOutOfRange(1001)), NumericSpec::with_precision(NonZero::new_unchecked(1001)));

            let spec = NumericSpec::with_precision(NonZero::new_unchecked(1000)).unwrap();
            assert_eq!(1000, spec.precision());
            assert_eq!(0, spec.scale());
        }
    }

    #[test]
    fn test_default() {
        let spec = NumericSpec::default();

        assert_eq!(NumericSpec::UNSPECIFIED_PRECISION, spec.precision());
        assert_eq!(NumericSpec::UNSPECIFIED_SCALE, spec.scale());
    }
}