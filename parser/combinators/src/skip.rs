pub fn skip(n: usize) -> SkipCombi {
    debug_assert_ne!(n, 0, "n must be greater than 0");
    SkipCombi(n)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SkipCombi(usize);

impl Combinator for SkipCombi {
    type Output = ();

    fn parse(&self, ctx: &mut ParserContext) -> scan::Result<Self::Output> {
        ctx.stream_mut().skip(self.0);
        Ok(())
    }
}

use crate::Combinator;
use crate::ParserContext;
use pg_parser_core::scan;
