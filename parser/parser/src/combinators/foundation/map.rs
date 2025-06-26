/// Maps the result of a parser combinator into another type.
pub(in crate::combinators) fn map_result<P, M, O>(parser: P, mapper: M) -> MapResultCombi<P, M, O>
where
    P: Combinator,
    M: Fn(scan::Result<P::Output>) -> scan::Result<O>
{
    MapResultCombi {
        parser,
        mapper,
        boo: PhantomData,
    }
}

/// Maps the `Ok(_)` value of a parser combinator into another type.
pub(in crate::combinators) fn map<P, M, O>(parser: P, mapper: M)
    -> MapResultCombi<
        P,
        impl Fn(scan::Result<P::Output>) -> scan::Result<O>,
        O
    >
where
    P: Combinator,
    M: Fn(P::Output) -> O
{
    // Reduces size of type names:
    fn inner<I, O>(mapper: impl Fn(I) -> O)
        -> impl Fn(scan::Result<I>) -> scan::Result<O>
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
pub(in crate::combinators) fn map_err<P, M>(parser: P, mapper: M)
    -> MapResultCombi<
        P,
        impl Fn(scan::Result<P::Output>) -> scan::Result<P::Output>,
        P::Output
    >
where
    P: Combinator,
    M: Fn(scan::Error) -> scan::Error
{
    // Reduces size of type names:
    fn inner<I>(mapper: impl Fn(scan::Error) -> scan::Error)
        -> impl Fn(scan::Result<I>) -> scan::Result<I>
    {
        move |result| result.map_err(&mapper)
    }

    MapResultCombi {
        parser,
        mapper: inner(mapper),
        boo: PhantomData,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct MapResultCombi<P, M, O> {
    parser: P,
    mapper: M,
    boo: PhantomData<O>
}

impl<P, M, O> Combinator for MapResultCombi<P, M, O>
where
    P: Combinator,
    M: Fn(scan::Result<P::Output>) -> scan::Result<O>
{
    type Output = O;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        (self.mapper)(self.parser.parse(stream))
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use core::marker::PhantomData;
