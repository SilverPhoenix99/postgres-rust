use crate::Named;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum BackslashQuote {
    Off,
    On,
    #[default]
    SafeEncoding,
}

impl Named for BackslashQuote {
    fn name(&self) -> &'static str {
        match self {
            Self::SafeEncoding => "safe_encoding",
            Self::Off => "off",
            Self::On => "on",
        }
    }
}
