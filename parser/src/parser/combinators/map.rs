/// Maps the result of a parser combinator into another type.
pub(in crate::parser) fn map_result<P, M, O>(parser: P, mapper: M) -> MapResultCombi<P, M, O>
where
    P: Combinator,
    M: Fn(ScanResult<P::Output>) -> ScanResult<O>
{
    MapResultCombi {
        parser,
        mapper,
        boo: PhantomData,
    }
}

/// Maps the `Ok(_)` value of a parser combinator into another type.
pub(in crate::parser) fn map<P, O>(parser: P, mapper: impl Fn(P::Output) -> O)
    -> impl Combinator<Output = O>
where
    P: Combinator
{
    // Reduces size of type names:
    fn inner<I, O>(mapper: impl Fn(I) -> O)
        -> impl Fn(ScanResult<I>)-> ScanResult<O>
    {
        move |result| result.map(&mapper)
    }

    MapResultCombi {
        parser,
        mapper: inner(mapper),
        boo: PhantomData,
    }
}

/// Maps the `Err(_)` value of a parser combinator into another type.
pub(in crate::parser) fn map_err<P>(parser: P, mapper: impl Fn(ScanErrorKind) -> ScanErrorKind)
    -> impl Combinator<Output = P::Output>
where
    P: Combinator
{
    // Reduces size of type names:
    fn inner<I>(mapper: impl Fn(ScanErrorKind) -> ScanErrorKind)
        -> impl Fn(ScanResult<I>) -> ScanResult<I>
    {
        move |result| result.map_err(&mapper)
    }

    MapResultCombi {
        parser,
        mapper: inner(mapper),
        boo: PhantomData,
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct MapResultCombi<P, M, O> {
    parser: P,
    mapper: M,
    boo: PhantomData<O>
}

impl<P, M, O> Combinator for MapResultCombi<P, M, O>
where
    P: Combinator,
    M: Fn(ScanResult<P::Output>) -> ScanResult<O>
{
    type Output = O;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        (self.mapper)(self.parser.parse(stream))
    }
}

impl<P, M, O> Debug for MapResultCombi<P, M, O>
where
    P: Combinator + Debug,
    M: Fn(ScanResult<P::Output>) -> ScanResult<O>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapResultCombi")
            .field("parser", &self.parser)
            .finish()
    }
}

use crate::parser::combinators::Combinator;
use crate::parser::result::{ScanErrorKind, ScanResult};
use crate::parser::token_stream::TokenStream;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
