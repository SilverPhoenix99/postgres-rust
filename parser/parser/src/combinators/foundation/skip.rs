pub(in crate::combinators) fn skip(n: usize) -> SkipCombi {
    debug_assert_ne!(n, 0, "n must be greater than 0");
    SkipCombi(n)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct SkipCombi(usize);

impl Combinator for SkipCombi {
    type Output = ();

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        stream.skip(self.0);
        Ok(())
    }
}

use crate::combinators::foundation::Combinator;
use crate::stream::TokenStream;
use pg_parser_core::scan;
