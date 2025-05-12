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
    use crate::parser::ast_node::SignedNumber::IntegerConst;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_large_object() {
        test_parser!(
            source = "large object 123",
            parser = large_object(),
            expected = IntegerConst(123)
        )
    }
}

use crate::parser::ast_node::SignedNumber;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::signed_number;
use postgres_parser_lexer::Keyword::Large;
use postgres_parser_lexer::Keyword::Object;
