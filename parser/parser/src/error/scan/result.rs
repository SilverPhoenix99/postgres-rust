pub(crate) type Result<T> = core::result::Result<T, scan::Error>;

impl<T> Required<T> for Result<T> {
    fn required(self) -> LocatedResult<T> {
        self.map_err(|err| match err {
            ScanErr(err) => err,
            NoMatch(loc) | ScanEof(loc) => syntax(loc)
        })
    }
}

impl<T> TryMatch<T> for Result<T> {
    fn try_match(self) -> LocatedResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch(_)) => Ok(None),
            Err(ScanEof(loc)) => Err(syntax(loc)),
            Err(ScanErr(err)) => Err(err),
        }
    }
}

impl<T> Optional<T> for Result<T> {
    fn optional(self) -> LocatedResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch(_) | ScanEof(_)) => Ok(None),
            Err(ScanErr(err)) => Err(err),
        }
    }
}

impl<T> MaybeMatch<T> for Result<T> {
    fn maybe_match(self) -> eof::Result<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch(_)) => Ok(None),
            Err(ScanEof(loc)) => Err(Eof(loc)),
            Err(ScanErr(err)) => Err(NotEof(err)),
        }
    }
}

use crate::eof;
use crate::eof::Error::Eof;
use crate::eof::Error::NotEof;
use crate::result::MaybeMatch;
use crate::result::Optional;
use crate::result::Required;
use crate::result::TryMatch;
use crate::scan;
use crate::scan::Error::Eof as ScanEof;
use crate::scan::Error::NoMatch;
use crate::scan::Error::ScanErr;
use pg_elog::syntax;
use pg_elog::LocatedResult;
