#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotEof(pg_elog::LocatedError),
    Eof(Location),
}

impl<T> From<Located<T>> for Error
where
    T: Into<pg_elog::Error>
{
    fn from(Located(source, loc): Located<T>) -> Self {
        NotEof(Located::new(source, loc))
    }
}

use crate::eof::Error::NotEof;
use pg_basics::Located;
use pg_basics::Location;
