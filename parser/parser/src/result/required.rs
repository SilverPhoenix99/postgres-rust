pub(crate) trait Required<T> {
    /// See [`required()`](crate::combinators::foundation::required::required).
    fn required(self) -> LocatedResult<T>;
}

use pg_elog::LocatedResult;
