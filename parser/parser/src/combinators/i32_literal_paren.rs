/// Production: `'(' ICONST ')'`
pub(super) fn i32_literal_paren(stream: &mut TokenStream) -> scan::Result<i32> {

    paren!(i32_literal).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_i32_literal_paren() {
        test_parser!(
            source = " (123 )",
            parser = i32_literal_paren,
            expected = 123
        )
    }
}

use crate::combinators::const_numeric::i32_literal;
use pg_combinators::paren;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
