pub(crate) trait Combinator {
    type Output;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output>;
}

impl<F, O> Combinator for F
where
    F: Fn(&mut TokenStream) -> Result<O>,
{
    type Output = O;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output> {
        self(stream)
    }
}

use crate::scan::Result;
use crate::stream::TokenStream;
