pub(in crate::combinators) trait CombinatorHelpers
where
    Self: Sized + Combinator
{
    /// See [`optional()`](optional::optional).
    #[inline]
    fn optional(self) -> OptionalCombi<Self> {
        optional(self)
    }

    /// See [`required()`](required::required).
    #[inline]
    fn required(self) -> RequiredCombi<Self> {
        required(self)
    }

    /// See [`try_match()`](try_match::try_match).
    #[inline]
    fn try_match(self) -> TryMatchCombi<Self> {
        try_match(self)
    }

    /// See [`maybe_match()`](maybe_match::maybe_match).
    #[inline]
    fn maybe_match(self) -> MaybeMatchCombi<Self> {
        maybe_match(self)
    }

    /// See [`and()`](and::and).
    #[inline]
    fn and<R>(self, right: R) -> AndCombi<Self, R>
    where
        R: Combinator
    {
        and(self, right)
    }

    /// Same as `(Self && R)`
    #[inline]
    fn and_then<R, O>(self, right: R, mapper: impl Fn(Self::Output, R::Output) -> O)
        -> impl Combinator<Output = O>
    where
        R: Combinator
    {
        self.and(right)
            .map(move |(left, right)| mapper(left, right))
    }

    /// Same as `(Self && R)`.
    ///
    /// Returns `Self::Output`.
    #[inline]
    fn and_left<R>(self, right: R) -> impl Combinator<Output = Self::Output>
    where
        R: Combinator
    {
        self.and(right).left()
    }

    /// Same as `(Self && R)`.
    ///
    /// Returns `Right::Output`.
    #[inline]
    fn and_right<R>(self, right: R) -> impl Combinator<Output = R::Output>
    where
        R: Combinator
    {
        self.and(right).right()
    }

    /// See [`or()`](or::or)
    #[inline]
    fn or<R>(self, right: R) -> OrCombi<Self, R>
    where
        R: Combinator<Output = Self::Output>
    {
        or(self, right)
    }

    /// See [`map()`](map::map).
    #[inline]
    fn map<O>(self, mapper: impl Fn(Self::Output) -> O) -> impl Combinator<Output = O>
    {
        map(self, mapper)
    }

    /// See [`map_err()`](map_err).
    #[inline]
    fn map_err(self, mapper: impl Fn(ScanErrorKind) -> ScanErrorKind)
        -> impl Combinator<Output = Self::Output>
    {
        map_err(self, mapper)
    }

    /// See [`map_result()`](map_result).
    #[inline]
    fn map_result<O>(self, mapper: impl Fn(ScanResult<Self::Output>) -> ScanResult<O>)
        -> impl Combinator<Output = O>
    {
        map_result(self, mapper)
    }

    #[inline]
    fn left<L, R>(self) -> impl Combinator<Output = L>
    where
        Self: Combinator<Output = (L, R)>
    {
        self.map(|(left, _)| left)
    }

    #[inline]
    fn right<L, R>(self) -> impl Combinator<Output = R>
    where
        Self: Combinator<Output = (L, R)>
    {
        self.map(|(_, right)| right)
    }

    #[inline]
    fn skip(self) -> SkipCombi<Self> {
        skip(self)
    }

    /// This is similar to [`CombinatorHelpers::map_result()`],
    /// but includes the stream as an argument to the closure.
    fn chain_result<O>(self, mapper: impl Fn(ScanResult<Self::Output>, &mut TokenStream) -> ScanResult<O>)
        -> impl Combinator<Output = O>
    {
        parser(move |stream| {
            let result = self.parse(stream);
            mapper(result, stream)
        })
    }

    /// This is similar to [`CombinatorHelpers::map()`],
    /// but includes the stream as an argument to the closure.
    fn chain<O>(self, mapper: impl Fn(Self::Output, &mut TokenStream) -> ScanResult<O>)
        -> impl Combinator<Output = O>
    {
        fn inner<I, O>(mapper: impl Fn(I, &mut TokenStream) -> ScanResult<O>)
            -> impl Fn(ScanResult<I>, &mut TokenStream) -> ScanResult<O>
        {
            move |result, stream| mapper(result?, stream)
        }

        let mapper= inner(mapper);
        self.chain_result(mapper)
    }

    /// This is similar to [`CombinatorHelpers::map_err()`],
    /// but includes the stream as an argument to the closure.
    fn chain_err(self, mapper: impl Fn(ScanErrorKind, &mut TokenStream) -> ScanResult<Self::Output>)
        -> impl Combinator<Output = Self::Output>
    {
        fn inner<I>(mapper: impl Fn(ScanErrorKind, &mut TokenStream) -> ScanResult<I>)
            -> impl Fn(ScanResult<I>, &mut TokenStream) -> ScanResult<I>
        {
            move |result, stream|
                match result {
                    Err(err) => mapper(err, stream),
                    _ => result,
                }
        }

        let mapper = inner(mapper);
        self.chain_result(mapper)
    }
}

impl<T: Combinator> CombinatorHelpers for T {}

use crate::combinators::foundation::and;
use crate::combinators::foundation::map;
use crate::combinators::foundation::map_err;
use crate::combinators::foundation::map_result;
use crate::combinators::foundation::maybe_match;
use crate::combinators::foundation::optional;
use crate::combinators::foundation::or;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::required;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::try_match;
use crate::combinators::foundation::AndCombi;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::MaybeMatchCombi;
use crate::combinators::foundation::OptionalCombi;
use crate::combinators::foundation::OrCombi;
use crate::combinators::foundation::RequiredCombi;
use crate::combinators::foundation::SkipCombi;
use crate::combinators::foundation::TryMatchCombi;
use crate::result::ScanErrorKind;
use crate::result::ScanResult;
use crate::stream::TokenStream;
