pub(super) fn large_object(stream: &mut TokenStream) -> scan::Result<SignedNumber> {

    /*
        LARGE OBJECT NumericOnly
    */

    let (.., id) = seq!(Large, Object, signed_number)
        .parse(stream)?;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::SignedNumber::IntegerConst;
    use pg_combinators::test_parser;

    #[test]
    fn test_large_object() {
        test_parser!(
            source = "large object 123",
            parser = large_object,
            expected = IntegerConst(123)
        )
    }
}

use crate::combinators::signed_number;
use pg_ast::SignedNumber;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Large;
use pg_lexer::Keyword::Object;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
