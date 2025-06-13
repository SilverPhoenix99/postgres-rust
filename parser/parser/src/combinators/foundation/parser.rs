/// Used to wrap combinators causing type names being too large.
macro_rules! enclosure {
    ($expr:expr) => {{
        let p = $expr;
        $crate::combinators::foundation::parser(move |stream| p.parse(stream))
    }};
}

pub(in crate::combinators) use enclosure;

pub(in crate::combinators) fn parser<F, T>(parser: F) -> ClosureCombi<F, T>
where
    F: Fn(&mut TokenStream) -> Result<T>
{
    ClosureCombi {
        parser,
        boo: PhantomData,
    }
}

pub(in crate::combinators) struct ClosureCombi<F, T> {
    parser: F,
    boo: PhantomData<T>,
}

impl<F, T> Combinator for ClosureCombi<F, T>
where
    F: Fn(&mut TokenStream) -> Result<T>
{
    type Output = T;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output> {
        (self.parser)(stream)
    }
}

impl<F, T> Debug for ClosureCombi<F, T>
where
    F: Fn(&mut TokenStream) -> Result<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ClosureCombi")
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenStream;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
