pub fn large_object(ctx: &mut ParserContext) -> scan::Result<SignedNumber> {

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
    use pg_combinators::test_parser;
    use pg_sink_ast::SignedNumber::IntegerConst;

    #[test]
    fn test_large_object() {
        test_parser!(
            source = "large object 123",
            parser = large_object,
            expected = IntegerConst(123)
        )
    }
}

use crate::signed_number;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Large;
use pg_lexer::Keyword::Object;
use pg_parser_core::scan;
use pg_sink_ast::SignedNumber;
