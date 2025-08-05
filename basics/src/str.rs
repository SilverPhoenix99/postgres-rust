#[derive(Debug, Clone, Eq, From)]
pub enum Str {
    #[from(Box<str>, String)]
    Boxed(Box<str>),
    #[from]
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

impl Display for Str {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Boxed(boxed) => f.write_str(boxed),
            Self::Static(string) => f.write_str(string),
        }
    }
}

use core::cmp::Ordering;
use core::fmt::Display;
use core::fmt::Formatter;
use core::ops::Deref;
use derive_more::From;
