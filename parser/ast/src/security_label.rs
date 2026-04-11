#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityLabel {
    provider: Option<Str>,
    label: Option<Box<str>>,
}

impl SecurityLabel {
    pub fn new(provider: Option<Str>, label: Option<Box<str>>) -> Self {
        Self { provider, label }
    }

    pub fn provider(&self) -> Option<&str> {
        self.provider.as_deref()
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

use pg_basics::Str;
