pub(crate) type ScanResult<T> = Result<T, ScanErrorKind>;

impl<T> Required<T> for ScanResult<T> {
    fn required(self) -> ParseResult<T> {
        self.map_err(|err| match err {
            ScanErr(err) => err,
            NoMatch(loc) | ScanEof(loc) => syntax(loc)
        })
    }
}

impl<T> TryMatch<T> for ScanResult<T> {
    fn try_match(self) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch(_)) => Ok(None),
            Err(ScanEof(loc)) => Err(syntax(loc)),
            Err(ScanErr(err)) => Err(err),
        }
    }
}

impl<T> Optional<T> for ScanResult<T> {
    fn optional(self) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch(_) | ScanEof(_)) => Ok(None),
            Err(ScanErr(err)) => Err(err),
        }
    }
}

impl<T> MaybeMatch<T> for ScanResult<T> {
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
use crate::parser::ParseResult;
use crate::result::MaybeMatch;
use crate::result::Optional;
use crate::result::Required;
use crate::result::ScanErrorKind;
use crate::result::ScanErrorKind::Eof as ScanEof;
use crate::result::ScanErrorKind::NoMatch;
use crate::result::ScanErrorKind::ScanErr;
use crate::result::TryMatch;
use pg_elog::syntax;
