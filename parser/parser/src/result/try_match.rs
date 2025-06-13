pub(crate) trait TryMatch<T> {
    /// See [`try_match()`](crate::combinators::foundation::try_match::try_match).
    fn try_match(self) -> LocatedResult<Option<T>>;
}

use pg_elog::LocatedResult;
