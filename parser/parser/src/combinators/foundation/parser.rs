pub(in crate::combinators) fn parser<F, O>(parser: F) -> ClosureCombi<F, O>
where
    F: Fn(&mut TokenStream) -> scan::Result<O>
{
    ClosureCombi {
        parser,
        boo: PhantomData,
    }
}

#[derive(Debug)]
pub(in crate::combinators) struct ClosureCombi<F, O> {
    parser: F,
    boo: PhantomData<O>,
}

impl<F, O> Combinator for ClosureCombi<F, O>
where
    F: Fn(&mut TokenStream) -> scan::Result<O>
{
    type Output = O;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        (self.parser)(stream)
    }
}

impl<F, O> From<F> for ClosureCombi<F, O>
where
    F: Fn(&mut TokenStream) -> scan::Result<O>
{
    fn from(parser: F) -> Self {
        Self { parser, boo: PhantomData }
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use core::marker::PhantomData;
