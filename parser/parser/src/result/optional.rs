pub(crate) trait Optional<T> {
    /// See [`optional()`](crate::combinators::foundation::optional::optional).
    fn optional(self) -> pg_elog::LocatedResult<Option<T>>;
}
