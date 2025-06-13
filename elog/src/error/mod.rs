pub(super) mod extended_string;
pub(super) mod lexer;
pub(super) mod located_error;
pub(super) mod parser;
pub(super) mod role_spec;
pub(super) mod unicode_string;

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
