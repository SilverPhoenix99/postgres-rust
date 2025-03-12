/// Used to wrap combinators causing type names being too large.
macro_rules! enclosure {
    ($expr:expr) => {
        $crate::parser::combinators::foundation::parser(move |stream| $expr.parse(stream))
    };
}
pub(in crate::parser::combinators) use enclosure;

pub(in crate::parser::combinators) fn parser<F, T>(parser: F) -> ClosureCombi<F, T>
where
    F: Fn(&mut TokenStream) -> ScanResult<T>
{
    ClosureCombi {
        parser,
        boo: PhantomData,
    }
}

pub(in crate::parser::combinators) struct ClosureCombi<F, T> {
    parser: F,
    boo: PhantomData<T>,
}

impl<F, T> Combinator for ClosureCombi<F, T>
where
    F: Fn(&mut TokenStream) -> ScanResult<T>
{
    type Output = T;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        (self.parser)(stream)
    }
}

impl<F, T> Debug for ClosureCombi<F, T>
where
    F: Fn(&mut TokenStream) -> ScanResult<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ClosureCombi")
    }
}

use crate::parser::combinators::foundation::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::TokenStream;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
