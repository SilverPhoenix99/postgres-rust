extern crate alloc;
extern crate core;

pg_basics::reexport! { pub
    result,
}

mod error;

pub mod eof {
    pub use crate::error::eof::error::*;
    pub use crate::error::eof::result::*;
}

pub mod scan {
    pub use crate::error::scan::error::*;
    pub use crate::error::scan::result::*;
}

pub fn syntax<T>(location: pg_basics::Location) -> T
where
    pg_elog::LocatedError: Into<T>
{
    pg_elog::LocatedError::new(pg_elog::parser::Error::Syntax, location).into()
}
