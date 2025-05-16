pub(crate) trait TryMatch<T> {
    /// See [`try_match()`](crate::combinators::foundation::try_match::try_match).
    fn try_match(self) -> ParseResult<Option<T>>;
}

use crate::parser::ParseResult;
