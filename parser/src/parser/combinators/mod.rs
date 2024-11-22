mod and;
mod between;
mod bit_string;
mod identifier;
mod integer;
mod keyword;
mod many;
mod map;
mod maybe_match;
mod number;
mod operator;
mod optional;
mod or;
mod parser;
mod required;
mod string;
mod try_match;
mod uescape_escape;
mod skip;

#[allow(unused_imports)]
pub(in crate::parser) use self::{
    and::{and, sequence, AndCombi},
    between::{between, BetweenCombi},
    bit_string::{bit_string, BitStringCombi},
    identifier::{identifier, IdentifierCombi},
    integer::{integer, IntegerCombi},
    keyword::{
        keyword,
        keyword_category,
        keyword_if,
        keyword_result,
        keyword_when,
        KeywordCategoryCombi,
        KeywordCombi,
        KeywordCondCombi,
    },
    many::{many, many_pre, many_sep, ManyCombi, ManyPrefixedCombi, ManySepCombi},
    map::{map, map_err, map_result},
    maybe_match::{maybe_match, MaybeMatchCombi},
    number::{number, NumberCombi},
    operator::{operator, operator_if, operator_result, operator_when, OperatorCombi, OperatorCondCombi},
    optional::{optional, OptionalCombi},
    or::{match_first, match_first_with_state, or, OrCombi},
    parser::{enclosure, parser, ClosureCombi},
    required::{required, RequiredCombi},
    skip::{skip, SkipCombi},
    string::{string, StringCombi},
    try_match::{try_match, TryMatchCombi}
};

pub(in crate::parser) trait Combinator: Debug {
    type Output;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output>;
}

pub(in crate::parser) trait CombinatorHelpers
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

    /// This is similar to [`CombinatorHelpers::map_result()`](CombinatorHelpers::map_result),
    /// but includes the stream as an argument to the closure.
    fn chain_result<O>(self, mapper: impl Fn(ScanResult<Self::Output>, &mut TokenStream) -> ScanResult<O>)
        -> impl Combinator<Output = O>
    {
        parser(move |stream| {
            let result = self.parse(stream);
            mapper(result, stream)
        })
    }

    /// This is similar to [`CombinatorHelpers::map()`](CombinatorHelpers::map),
    /// but includes the stream as an argument to the closure.
    fn chain<O>(self, mapper: impl Fn(Self::Output, &mut TokenStream) -> O)
        -> impl Combinator<Output = O>
    {
        fn inner<I, O>(mapper: impl Fn(I, &mut TokenStream) -> O)
            -> impl Fn(ScanResult<I>, &mut TokenStream) -> ScanResult<O>
        {
            move |result, stream| {
                let result = result?;
                let ok = mapper(result, stream);
                Ok(ok)
            }
        }

        self.chain_result(inner(mapper))
    }

    /// This is similar to [`CombinatorHelpers::map_err()`](CombinatorHelpers::map_err),
    /// but includes the stream as an argument to the closure.
    fn chain_err(self, mapper: impl Fn(ScanErrorKind, &mut TokenStream) -> ScanErrorKind)
        -> impl Combinator
    {
        fn inner<O>(mapper: impl Fn(ScanErrorKind, &mut TokenStream) -> ScanErrorKind)
            -> impl Fn(ScanResult<O>, &mut TokenStream) -> ScanResult<O>
        {
            move |result, stream| {
                result.map_err(|err|
                    mapper(err, stream)
                )
            }
        }

        self.chain_result(inner(mapper))
    }
}

impl<T: Combinator> CombinatorHelpers for T {}

use crate::parser::result::{ScanErrorKind, ScanResult};
use crate::parser::token_stream::TokenStream;
use std::fmt::Debug;
