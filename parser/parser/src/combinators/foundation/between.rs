#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct BetweenCombi<L, P, R> {
    left: L,
    parser: P,
    right: R,
}

pub(in crate::combinators) fn between<L, P, R>(
    left: L,
    parser: P,
    right: R
)
    -> BetweenCombi<L, P, R>
where
    L: Combinator,
    P: Combinator,
    R: Combinator,
{
    BetweenCombi { left, parser, right }
}

impl<L, P, R> Combinator for BetweenCombi<L, P, R>
where
    L: Combinator,
    P: Combinator,
    R: Combinator,
{
    type Output = P::Output;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output> {
        self.left.parse(stream)?;
        let result = self.parser.parse(stream).required()?;
        self.right.parse(stream).required()?;
        Ok(result)
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::Required;
use crate::scan::Result;
use crate::stream::TokenStream;
