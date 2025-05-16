pub(super) fn large_object() -> impl Combinator<Output = SignedNumber> {

    /*
        LARGE OBJECT NumericOnly
    */

    and(Large, Object)
        .and_right(signed_number())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use postgres_parser_ast::SignedNumber::IntegerConst;

    #[test]
    fn test_large_object() {
        test_parser!(
            source = "large object 123",
            parser = large_object(),
            expected = IntegerConst(123)
        )
    }
}

use crate::combinators::foundation::and;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::signed_number;
use postgres_parser_ast::SignedNumber;
use postgres_parser_lexer::Keyword::Large;
use postgres_parser_lexer::Keyword::Object;
