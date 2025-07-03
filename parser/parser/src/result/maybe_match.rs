pub(crate) trait MaybeMatch<T> {
    /// `NoMatch` becomes `Ok(None)`.
    fn maybe_match(self) -> eof::Result<Option<T>>;
}

use crate::eof;
