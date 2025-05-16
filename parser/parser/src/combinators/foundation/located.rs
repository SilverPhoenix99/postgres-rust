pub(in crate::combinators) fn located<P>(parser: P) -> LocCombi<P>
where
    P: Combinator
{
    LocCombi { parser }
}

#[derive(Debug)]
pub(in crate::combinators) struct LocCombi<P>
where
    P: Combinator
{
    parser: P,
}

impl<P> Combinator for LocCombi<P>
where
    P: Combinator
{
    type Output = Located<P::Output>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {

        let loc = stream.current_location();
        self.parser.parse(stream)
            .map(|ok| (ok, loc))
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::ScanResult;
use crate::stream::TokenStream;
use pg_basics::Located;
