/// Hoists `NoMatch` to `Ok(None)`.
///
/// Usually used when the 1st token is optional,
/// or there are multiple rules in the production,
/// but it should still break the whole production on `Eof` and `ParserErr`.
pub(in crate::combinators) fn maybe_match<P>(parser: P) -> MaybeMatchCombi<P>
where
    P: Combinator
{
    MaybeMatchCombi(parser)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct MaybeMatchCombi<P>(P);

impl<P> Combinator for MaybeMatchCombi<P>
where
    P: Combinator
{
    type Output = Option<P::Output>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output> {
        self.0.parse(stream)
            .maybe_match()
            .map_err(Error::from)
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::MaybeMatch;
use crate::scan::Error;
use crate::scan::Result;
use crate::stream::TokenStream;
