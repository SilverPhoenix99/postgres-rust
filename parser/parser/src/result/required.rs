pub(crate) trait Required<T> {
    /// See [`required()`](crate::combinators::foundation::required::required).
    fn required(self) -> ParseResult<T>;
}

use crate::parser::ParseResult;
