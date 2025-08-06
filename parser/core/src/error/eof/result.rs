pub type Result<T> = core::result::Result<T, eof::Error>;

impl<T> Required<T> for Result<T> {
    fn required(self) -> pg_elog::LocatedResult<T> {
        self.map_err(|err| match err {
            NotEof(err) => err,
            Eof(loc) => syntax(loc)
        })
    }
}

impl<T> Optional<T> for Result<T> {
    fn optional(self) -> pg_elog::LocatedResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(Eof(_)) => Ok(None),
            Err(NotEof(err)) => Err(err),
        }
    }
}

use crate::eof;
use crate::eof::Error::Eof;
use crate::eof::Error::NotEof;
use crate::syntax;
use crate::Optional;
use crate::Required;
