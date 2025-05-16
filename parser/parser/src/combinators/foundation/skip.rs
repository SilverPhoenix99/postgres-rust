pub(in crate::combinators) fn skip<P>(parser: P) -> SkipCombi<P>
where
    P: Combinator
{
    SkipCombi(parser)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct SkipCombi<P>(P);

impl<P> Combinator for SkipCombi<P>
where
    P: Combinator
{
    type Output = ();

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        self.0.parse(stream)?;
        Ok(())
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::ScanResult;
use crate::stream::TokenStream;
