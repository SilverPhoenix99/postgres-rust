pub(crate) trait Optional<T> {
    /// See [`optional()`](crate::combinators::foundation::optional::optional).
    fn optional(self) -> LocatedResult<Option<T>>;
}

use pg_elog::LocatedResult;
