extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;
extern crate core;

mod combinators;
mod error;
mod result;
mod stream;
mod tests;

mod eof {
    pub(crate) use crate::error::eof::error::*;
    pub(crate) use crate::error::eof::result::*;
}

mod scan {
    pub(crate) use crate::error::scan::error::*;
    pub(crate) use crate::error::scan::result::*;
}

pg_basics::reexport! { pub
    config,
    parser,
}

fn syntax<T>(location: pg_basics::Location) -> T
where
    pg_elog::LocatedError: Into<T>
{
    pg_elog::LocatedError::new(pg_elog::parser::Error::Syntax, location).into()
}

fn no_match<T>(stream: &mut stream::TokenStream) -> scan::Result<T> {
    Err(scan::Error::NoMatch(stream.current_location()))
}
