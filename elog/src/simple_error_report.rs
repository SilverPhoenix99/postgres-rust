#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SimpleErrorReport<T> {
    source: T
}

impl<T> SimpleErrorReport<T> {
    #[inline(always)]
    pub fn new(source: T) -> Self {
        Self { source }
    }

    #[inline(always)]
    pub fn source(&self) -> &T {
        &self.source
    }
}

impl<T: Error + 'static> Error for SimpleErrorReport<T> {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl<T: Error> Display for SimpleErrorReport<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.source, f)
    }
}

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
