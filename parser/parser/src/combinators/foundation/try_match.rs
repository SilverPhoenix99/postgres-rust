/// `Eof` becomes `Err(Syntax)`.
///
/// `NoMatch` becomes `Ok(None)`.
pub(in crate::combinators) fn try_match<P>(parser: P) -> TryMatchCombi<P>
where
    P: Combinator
{
    TryMatchCombi(parser)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct TryMatchCombi<P>(P);

impl<P> Combinator for TryMatchCombi<P>
where
    P: Combinator
{
    type Output = Option<P::Output>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output> {
        self.0.parse(stream)
            .try_match()
            .map_err(Error::from)
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::TryMatch;
use crate::scan::Error;
use crate::scan::Result;
use crate::stream::TokenStream;
