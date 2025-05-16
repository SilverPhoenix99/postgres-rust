pub(crate) trait Optional<T> {
    /// See [`optional()`](crate::combinators::foundation::optional::optional).
    fn optional(self) -> ParseResult<Option<T>>;
}

use crate::parser::ParseResult;
