extern crate alloc;
#[cfg(test)]
#[macro_use]
extern crate assert_matches;
extern crate core;

pg_basics::reexport! { pub
    config,
    result,
}

pub mod scan {
    pub use crate::error::scan::error::*;
    pub use crate::error::scan::result::*;
}

pub mod stream;

mod error;

mod eof {
    pub(crate) use crate::error::eof::error::*;
    pub(crate) use crate::error::eof::result::*;
}

pub fn syntax<T>(location: pg_basics::Location) -> T
where
    pg_elog::LocatedError: Into<T>
{
    pg_elog::LocatedError::new(pg_elog::parser::Error::Syntax, location).into()
}
