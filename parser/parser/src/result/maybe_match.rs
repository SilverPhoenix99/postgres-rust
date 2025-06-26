pub(crate) trait MaybeMatch<T> {
    /// See [`maybe_match()`](crate::combinators::foundation::maybe_match::maybe_match).
    fn maybe_match(self) -> eof::Result<Option<T>>;
}

use crate::eof;
