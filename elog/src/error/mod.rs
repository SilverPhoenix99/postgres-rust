pub(super) mod lexer;
pub(super) mod role_spec;

mod located_error;

pub use located_error::LocatedError;

pub trait Error: core::error::Error {

    fn sql_state(&self) -> SqlState;

    #[inline(always)]
    fn hint(&self) -> Option<Str> {
        None
    }

    #[inline(always)]
    fn detail(&self) -> Option<Str> {
        None
    }

    #[inline(always)]
    fn detail_log(&self) -> Option<Str> {
        None
    }
}

use crate::SqlState;
use pg_basics::Str;
