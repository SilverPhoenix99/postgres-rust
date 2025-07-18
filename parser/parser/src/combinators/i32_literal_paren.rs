/// Production: `'(' ICONST ')'`
pub(super) fn i32_literal_paren(stream: &mut TokenStream) -> scan::Result<i32> {

    between_paren(integer)
        .parse(stream)
        .map(i32::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_i32_literal_paren() {
        test_parser!(
            source = " (123 )",
            parser = i32_literal_paren,
            expected = 123
        )
    }
}

use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::integer;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
