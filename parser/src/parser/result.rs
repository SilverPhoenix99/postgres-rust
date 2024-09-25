/// The default result type for most productions.
///
/// `Ok(None)` and `Err(None)` are soft errors: there's a valid token
/// that didn't match the criteria, or Eof, respectively.
///
/// It can be converted to `ReqResult` by calling `.required()`.
///
/// * `Ok(Some(_))`  : The token matched the expected one.
/// * `Ok(None)`     : (No match) There's a token, but it doesn't match the expected one(s).
///                    Might be recoverable if the token was optional.
/// * `Err(Some(_))` : Unrecoverable error.
/// * `Err(None)`    : Eof. Might be recoverable if the token was optional.
pub type OptResult<T, E = ParserError> = Result<Option<T>, Option<E>>;

/// Used when a result must exist, and when otherwise it's considered a `Syntax` error.
pub type ReqResult<T, E = ParserError> = Result<T, Option<E>>;

pub(super) trait OptionalResult<T, E = ParserError>: Sized
where
    E: Default
{
    fn eof(&self) -> bool;
    fn no_match(&self) -> bool;

    /// Maps an optional production into a required one,
    /// by converting No Match (`Ok(Some)`) and Eof (`Err(None)`)
    /// into a Syntax error.
    ///
    /// #### Warning:
    /// This method is lossy.
    /// After converting to a "Required" result,
    /// it won't be possible to distinguish between
    /// No Match (`Ok(Some)`) and Eof (`Err(None)`).
    #[inline(always)]
    fn required(self) -> ReqResult<T, E> {
        self.replace_none(Err(Some(E::default())))
    }

    fn map_eof(self, map: impl FnOnce() -> OptResult<T, E>) -> OptResult<T, E>;
    fn map_no_match(self, map: impl FnOnce() -> OptResult<T, E>) -> OptResult<T, E>;
    fn map_none(self, map: impl FnOnce() -> ReqResult<T, E>) -> ReqResult<T, E>;

    #[inline(always)]
    fn replace_eof(self, replacement: OptResult<T, E>) -> OptResult<T, E> {
        self.map_eof(|| replacement)
    }

    #[inline(always)]
    fn replace_no_match(self, replacement: OptResult<T, E>) -> OptResult<T, E> {
        self.map_no_match(|| replacement)
    }

    #[inline(always)]
    fn replace_none(self, replacement: ReqResult<T, E>) -> ReqResult<T, E> {
        self.map_none(|| replacement)
    }
}

impl<T, E> OptionalResult<T, E> for OptResult<T, E>
where
    E: Default
{
    #[inline(always)]
    fn eof(&self) -> bool {
        matches!(self, Err(None))
    }

    #[inline(always)]
    fn no_match(&self) -> bool {
        matches!(self, Ok(None))
    }

    #[inline(always)]
    fn map_eof(self, map: impl FnOnce() -> OptResult<T, E>) -> OptResult<T, E> {
        match self {
            Err(None) => map(),
            res => res
        }
    }

    #[inline(always)]
    fn map_no_match(self, map: impl FnOnce() -> OptResult<T, E>) -> OptResult<T, E> {
        match self {
            Ok(None) => map(),
            res => res,
        }
    }

    #[inline(always)]
    fn map_none(self, map: impl FnOnce() -> ReqResult<T, E>) -> ReqResult<T, E> {
        match self {
            Ok(None) | Err(None) => map(),
            Ok(Some(res)) => Ok(res),
            Err(res) => Err(res),
        }
    }
}

pub(super) trait RequiredResult<T, E> {

    fn optional(self) -> OptResult<T, E>;
}

impl<T, E> RequiredResult<T, E> for ReqResult<T, E> {

    /// **Warning**: This method never returns `Ok(None)` or `Err(None)`,
    /// it just wraps the value or error in `Some(_)`.
    ///
    /// This is just a convenience method to convert a `ReqResult` to an `OptResult`.
    #[inline(always)]
    fn optional(self) -> OptResult<T, E> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type O = Option<usize>;

    #[test]
    fn test_eof() {
        assert!(!Ok::<O, O>(Some(1)).eof());
        assert!(!Ok::<O, O>(None).eof());
        assert!(!Err::<O, O>(Some(1)).eof());
        assert!(Err::<O, O>(None).eof());
    }

    #[test]
    fn test_no_match() {
        assert!(!Ok::<O, O>(Some(1)).no_match());
        assert!(Ok::<O, O>(None).no_match());
        assert!(!Err::<O, O>(Some(1)).no_match());
        assert!(!Err::<O, O>(None).no_match());
    }

    #[test]
    fn test_map_eof() {
        assert_eq!(
            Ok(Some(1)),
            Ok(Some(1)).map_eof(|| Err(Some(1)) )
        );
        assert_eq!(
            Ok(None),
            Ok::<O, O>(None).map_eof(|| Err(Some(1)) )
        );
        assert_eq!(
            Err(Some(1)),
            Err(Some(1)).map_eof(|| Ok(Some(1)) )
        );
        assert_eq!(
            Ok(Some(1)),
            Err::<O, O>(None).map_eof(|| Ok(Some(1)) )
        );
    }

    #[test]
    fn test_map_no_match() {
        assert_eq!(
            Ok(Some(1)),
            Ok(Some(1)).map_no_match(|| Err(Some(1)) )
        );
        assert_eq!(
            Err(Some(1)),
            Ok::<O, O>(None).map_no_match(|| Err(Some(1)) )
        );
        assert_eq!(
            Err(Some(1)),
            Err(Some(1)).map_no_match(|| Ok(Some(1)) )
        );
        assert_eq!(
            Err(None),
            Err::<O, O>(None).map_no_match(|| Ok(Some(1)) )
        );
    }

    #[test]
    fn test_map_none() {
        assert_eq!(
            Ok(1),
            Ok(Some(1)).map_none(|| Err(Some(1)) )
        );
        assert_eq!(
            Err(Some(1)),
            Ok::<O, O>(None).map_none(|| Err(Some(1)) )
        );
        assert_eq!(
            Err(Some(1)),
            Err(Some(1)).map_none(|| Ok(1) )
        );
        assert_eq!(
            Ok(1),
            Err::<O, O>(None).map_none(|| Ok(1) )
        );
    }

    #[test]
    fn test_replace_eof() {
        assert_eq!(
            Ok(Some(1)),
            Ok(Some(1)).replace_eof(Err(Some(1)))
        );
        assert_eq!(
            Ok(None),
            Ok::<O, O>(None).replace_eof(Err(Some(1)))
        );
        assert_eq!(
            Err(Some(1)),
            Err(Some(1)).replace_eof(Ok(Some(1)))
        );
        assert_eq!(
            Ok(Some(1)),
            Err::<O, O>(None).replace_eof(Ok(Some(1)))
        );
    }

    #[test]
    fn test_replace_no_match() {
        assert_eq!(
            Ok(Some(1)),
            Ok(Some(1)).replace_no_match(Err(Some(1)))
        );
        assert_eq!(
            Err(Some(1)),
            Ok::<O, O>(None).replace_no_match(Err(Some(1)))
        );
        assert_eq!(
            Err(Some(1)),
            Err(Some(1)).replace_no_match(Ok(Some(1)))
        );
        assert_eq!(
            Err(None),
            Err::<O, O>(None).replace_no_match(Ok(Some(1)))
        );
    }

    #[test]
    fn test_replace_none() {
        assert_eq!(
            Ok(1),
            Ok(Some(1)).replace_none(Err(Some(1)))
        );
        assert_eq!(
            Err(Some(1)),
            Ok::<O, O>(None).replace_none(Err(Some(1)))
        );
        assert_eq!(
            Err(Some(1)),
            Err(Some(1)).replace_none(Ok(1))
        );
        assert_eq!(
            Ok(1),
            Err::<O, O>(None).replace_none(Ok(1))
        );
    }

    #[test]
    fn test_required() {
        assert_eq!(Ok(1),        Ok::<O, O>(Some(1)).required());
        assert_eq!(Err(Some(0)), Ok::<O, O>(None).required());
        assert_eq!(Err(Some(1)), Err::<O, O>(Some(1)).required());
        assert_eq!(Err(Some(0)), Err::<O, O>(None).required());
    }

    #[test]
    fn test_optional() {
        assert_eq!(Ok(Some(1)), Ok::<usize, O>(1).optional());
        assert_eq!(Err(Some(1)), Err::<usize, O>(Some(1)).optional());
        assert_eq!(Err(None), Err::<usize, O>(None).optional());
    }
}

use crate::parser::ParserError;
