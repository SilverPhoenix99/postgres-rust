pub(crate) trait Required<T> {
    /// See [`required()`](crate::combinators::foundation::required::required).
    fn required(self) -> pg_elog::LocatedResult<T>;
}
