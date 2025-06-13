pub(crate) mod error;
pub(crate) mod warning;

pub type LocatedError = crate::LocatedError<error::Error>;
