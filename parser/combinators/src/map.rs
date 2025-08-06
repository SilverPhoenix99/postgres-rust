/// Maps the `Ok(_)` value of a parser combinator into another type.
pub fn map<P, M, O>(parser: P, mapper: M) -> MapCombi<P, M, O>
where
    P: Combinator,
    M: Fn(P::Output) -> O
{
    MapCombi {
        parser,
        mapper,
        boo: PhantomData,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MapCombi<P, M, O> {
    parser: P,
    mapper: M,
    boo: PhantomData<O>
}

impl<P, M, O> Combinator for MapCombi<P, M, O>
where
    P: Combinator,
    M: Fn(P::Output) -> O
{
    type Output = O;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        let output = self.parser.parse(stream)?;
        let output = (self.mapper)(output);
        Ok(output)
    }
}

use crate::Combinator;
use core::marker::PhantomData;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
