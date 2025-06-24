/// Production: `'(' ICONST ')'`
pub(super) fn i32_literal_paren(stream: &mut TokenStream) -> Result<i32> {

    between!(paren : stream =>
        integer.parse(stream)
            .map(From::from)
    )
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

use crate::combinators::foundation::between;
use crate::combinators::foundation::integer;
use crate::scan::Result;
use crate::stream::TokenStream;
