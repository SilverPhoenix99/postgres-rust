#[derive(Debug, Clone, Eq)]
pub enum Str {
    Boxed(Box<str>),
    Static(&'static str),
}

impl Str {
    pub fn from_cloning(string: &str) -> Str {
        Str::Boxed(string.into())
    }
}

impl Default for Str {
    fn default() -> Str {
        Str::Static("")
    }
}

impl Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsRef<str> for Str {
    fn as_ref(&self) -> &str {
        match self {
            Str::Boxed(boxed) => boxed,
            Str::Static(string) => string,
        }
    }
}

impl_from!(String for Str::Boxed);

impl From<Box<str>> for Str {
    fn from(value: Box<str>) -> Self {
        Self::Boxed(value)
    }
}

impl From<&'static str> for Str {
    fn from(value: &'static str) -> Self {
        Self::Static(value)
    }
}

impl PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Ord for Str {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl PartialOrd for Str {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use crate::impl_from;
use core::cmp::Ordering;
use core::ops::Deref;
