pub type Result<T> = core::result::Result<T, scan::Error>;

impl<T> Required<T> for Result<T> {
    fn required(self) -> pg_elog::LocatedResult<T> {
        self.map_err(|err| match err {
            ScanErr(err) => err,
            NoMatch(loc) | ScanEof(loc) => syntax(loc)
        })
    }
}

impl<T> Optional<T> for Result<T> {
    fn optional(self) -> pg_elog::LocatedResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(NoMatch(_) | ScanEof(_)) => Ok(None),
            Err(ScanErr(err)) => Err(err),
        }
    }
}

use crate::scan;
use crate::scan::Error::Eof as ScanEof;
use crate::scan::Error::NoMatch;
use crate::scan::Error::ScanErr;
use crate::syntax;
use crate::Optional;
use crate::Required;
