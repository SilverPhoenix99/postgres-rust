pub(in crate::combinators::stmt) fn large_object(ctx: &mut ParserContext) -> scan::Result<SignedNumber> {

    /*
        LARGE OBJECT NumericOnly
    */

    let (.., id) = seq!(Large, Object, signed_number)
        .parse(ctx)?;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::SignedNumber::IntegerConst;

    #[test]
    fn test_large_object() {
        test_parser!(
            source = "large object 123",
            parser = large_object,
            expected = IntegerConst(123)
        )
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::signed_number;
use crate::seq;
use crate::ParserContext;
use pg_ast::SignedNumber;
use pg_lexer::Keyword::Large;
use pg_lexer::Keyword::Object;
use pg_parser_core::scan;
