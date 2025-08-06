pub trait Optional<T> {
    /// See [`optional()`](crate::combinators::foundation::optional::optional).
    fn optional(self) -> pg_elog::LocatedResult<Option<T>>;
}

pub trait Required<T> {
    /// See [`required()`](crate::combinators::foundation::required::required).
    fn required(self) -> pg_elog::LocatedResult<T>;
}
