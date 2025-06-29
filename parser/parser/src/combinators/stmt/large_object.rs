pub(super) fn large_object(stream: &mut TokenStream) -> scan::Result<SignedNumber> {

    /*
        LARGE OBJECT NumericOnly
    */

    let (.., id) = (Large, Object, signed_number)
        .parse(stream)?;

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
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

use crate::combinators::foundation::Combinator;
use crate::combinators::signed_number;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::SignedNumber;
use pg_lexer::Keyword::Large;
use pg_lexer::Keyword::Object;
