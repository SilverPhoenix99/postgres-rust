pub(super) fn typecast() -> impl Combinator<Output = Typecast> {

    /*
        CAST '(' Typename AS Typename ')'
    */

    Cast.and_right(between_paren(
        typename().and_then(
            As.and_right(typename()),
            Typecast::new
        )
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use postgres_parser_ast::TypeName::Int4;
    use postgres_parser_ast::TypeName::Int8;

    #[test]
    fn test_typecast() {
        test_parser!(
            source = "cast (int as bigint)",
            parser = typecast(),
            expected = Typecast::new(Int4, Int8)
        )
    }
}

use crate::combinators::between_paren;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::typename;
use postgres_parser_ast::Typecast;
use postgres_parser_lexer::Keyword::As;
use postgres_parser_lexer::Keyword::Cast;
