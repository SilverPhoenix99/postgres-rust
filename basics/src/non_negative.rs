#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Into)]
pub struct NonNegative(i32);

impl NonNegative {
    pub fn unwrap(&self) -> u32 {
        // SAFETY: `self.0` is already verified to be between 0..=i32::MAX.
        self.0.try_into().unwrap()
    }
}

impl From<u32> for NonNegative {
    fn from(value: u32) -> Self {
        assert!(value <= i32::MAX as u32, "non-negative integer overflow");
        Self(value as i32)
    }
}

impl From<i32> for NonNegative {
    fn from(value: i32) -> Self {
        assert!(value >= 0, "value must be a non-negative integer: {}", value);
        Self(value)
    }
}

impl From<NonNegative> for u32 {
    fn from(value: NonNegative) -> Self {
        value.0 as u32
    }
}

use derive_more::Into;
