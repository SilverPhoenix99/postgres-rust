/// Production: `'(' ICONST ')'`
pub(super) fn i32_literal_paren(ctx: &mut ParserContext) -> scan::Result<i32> {

    paren!(i32_literal).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_i32_literal_paren() {
        test_parser!(
            source = " (123 )",
            parser = i32_literal_paren,
            expected = 123
        )
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::i32_literal;
use crate::paren;
use crate::ParserContext;
use pg_parser_core::scan;
