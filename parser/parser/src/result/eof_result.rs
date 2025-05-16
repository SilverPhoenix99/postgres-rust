pub(crate) type EofResult<T> = Result<T, EofErrorKind>;

impl<T> Required<T> for EofResult<T> {
    fn required(self) -> ParseResult<T> {
        self.map_err(|err| match err {
            NotEof(err) => err,
            Eof(loc) => ParserError::syntax(loc)
        })
    }
}

impl<T> TryMatch<T> for EofResult<T> {
    fn try_match(self) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(Eof(loc)) => Err(ParserError::syntax(loc)),
            Err(NotEof(err)) => Err(err),
        }
    }
}

impl<T> Optional<T> for EofResult<T> {
    fn optional(self) -> ParseResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(Eof(_)) => Ok(None),
            Err(NotEof(err)) => Err(err),
        }
    }
}

use crate::parser::ParseResult;
use crate::result::EofErrorKind;
use crate::result::EofErrorKind::Eof;
use crate::result::EofErrorKind::NotEof;
use crate::result::Optional;
use crate::result::Required;
use crate::result::TryMatch;
use pg_elog::parser::ParserError;
