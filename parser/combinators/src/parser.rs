pub fn parser<F, O>(parser: F) -> ClosureCombi<F, O>
where
    F: Fn(&mut ParserContext) -> scan::Result<O>
{
    ClosureCombi {
        parser,
        boo: PhantomData,
    }
}

#[derive(Debug)]
pub struct ClosureCombi<F, O> {
    parser: F,
    boo: PhantomData<O>,
}

impl<F, O> Combinator for ClosureCombi<F, O>
where
    F: Fn(&mut ParserContext) -> scan::Result<O>
{
    type Output = O;

    fn parse(&self, ctx: &mut ParserContext<'_>) -> scan::Result<Self::Output> {
        (self.parser)(ctx)
    }
}

impl<F, O> From<F> for ClosureCombi<F, O>
where
    F: Fn(&mut ParserContext) -> scan::Result<O>
{
    fn from(parser: F) -> Self {
        Self { parser, boo: PhantomData }
    }
}

use crate::Combinator;
use crate::ParserContext;
use core::marker::PhantomData;
use pg_parser_core::scan;
