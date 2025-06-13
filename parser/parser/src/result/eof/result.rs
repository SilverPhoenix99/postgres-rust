pub(crate) type Result<T> = core::result::Result<T, Error>;

impl<T> Required<T> for Result<T> {
    fn required(self) -> LocatedResult<T> {
        self.map_err(|err| match err {
            NotEof(err) => err,
            Eof(loc) => syntax(loc)
        })
    }
}

impl<T> TryMatch<T> for Result<T> {
    fn try_match(self) -> LocatedResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(Eof(loc)) => Err(syntax(loc)),
            Err(NotEof(err)) => Err(err),
        }
    }
}

impl<T> Optional<T> for Result<T> {
    fn optional(self) -> LocatedResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(Eof(_)) => Ok(None),
            Err(NotEof(err)) => Err(err),
        }
    }
}

use crate::eof::Error;
use crate::eof::Error::Eof;
use crate::eof::Error::NotEof;
use crate::result::Optional;
use crate::result::Required;
use crate::result::TryMatch;
use pg_elog::syntax;
use pg_elog::LocatedResult;
