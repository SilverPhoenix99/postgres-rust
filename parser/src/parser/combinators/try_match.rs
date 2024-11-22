/// `Eof` becomes `Err(Syntax)`.
///
/// `NoMatch` becomes `Ok(None)`.
pub(in crate::parser) fn try_match<P>(parser: P) -> TryMatchCombi<P>
where
    P: Combinator
{
    TryMatchCombi(parser)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct TryMatchCombi<P>(P);

impl<P> Combinator for TryMatchCombi<P>
where
    P: Combinator
{
    type Output = Option<P::Output>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        self.0.parse(stream)
            .try_match()
            .map_err(ScanErrorKind::from)
    }
}

use crate::parser::combinators::Combinator;
use crate::parser::result::{ScanErrorKind, ScanResult, TryMatch};
use crate::parser::token_stream::TokenStream;
