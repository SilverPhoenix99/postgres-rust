/// Used to wrap combinators causing type names being too large.
macro_rules! enclosure {
    ($expr:expr) => {{
        let p = $expr;
        $crate::combinators::foundation::parser(move |stream| p.parse(stream))
    }};
}

pub(in crate::combinators) use enclosure;

pub(in crate::combinators) fn parser<F, O>(parser: F) -> ClosureCombi<F, O>
where
    F: Fn(&mut TokenStream) -> Result<O>
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
    F: Fn(&mut TokenStream) -> Result<O>
{
    type Output = O;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output> {
        (self.parser)(stream)
    }
}

impl<F, O> From<F> for ClosureCombi<F, O>
where
    F: Fn(&mut TokenStream) -> Result<O>
{
    fn from(parser: F) -> Self {
        Self { parser, boo: PhantomData }
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenStream;
use core::marker::PhantomData;
