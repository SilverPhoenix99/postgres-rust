/// Maps the result of a parser combinator into another type.
pub(in crate::combinators) fn map_result<P, M, O>(parser: P, mapper: M) -> MapResultCombi<P, M, O>
where
    P: Combinator,
    M: Fn(Result<P::Output>) -> Result<O>
{
    MapResultCombi {
        parser,
        mapper,
        boo: PhantomData,
    }
}

/// Maps the `Ok(_)` value of a parser combinator into another type.
pub(in crate::combinators) fn map<P, O>(parser: P, mapper: impl Fn(P::Output) -> O)
    -> impl Combinator<Output = O>
where
    P: Combinator
{
    // Reduces size of type names:
    fn inner<I, O>(mapper: impl Fn(I) -> O)
        -> impl Fn(Result<I>)-> Result<O>
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
pub(in crate::combinators) fn map_err<P>(parser: P, mapper: impl Fn(Error) -> Error)
    -> impl Combinator<Output = P::Output>
where
    P: Combinator
{
    // Reduces size of type names:
    fn inner<I>(mapper: impl Fn(Error) -> Error)
        -> impl Fn(Result<I>) -> Result<I>
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
pub(in crate::combinators) struct MapResultCombi<P, M, O> {
    parser: P,
    mapper: M,
    boo: PhantomData<O>
}

impl<P, M, O> Combinator for MapResultCombi<P, M, O>
where
    P: Combinator,
    M: Fn(Result<P::Output>) -> Result<O>
{
    type Output = O;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output> {
        (self.mapper)(self.parser.parse(stream))
    }
}

impl<P, M, O> Debug for MapResultCombi<P, M, O>
where
    P: Combinator + Debug,
    M: Fn(Result<P::Output>) -> Result<O>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapResultCombi")
            .field("parser", &self.parser)
            .finish()
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan::Error;
use crate::scan::Result;
use crate::stream::TokenStream;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
