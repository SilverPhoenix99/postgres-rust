pub(super) fn typecast() -> impl Combinator<Output = Typecast> {

    /*
        CAST '(' Typename AS Typename ')'
    */

    Cast.and_right(between_paren(
        sequence!(
            typename(),
            As,
            typename()
        )
            .map(|(from_type, _, to_type)|
                Typecast::new(from_type, to_type)
            )
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::TypeName::Int4;
    use pg_ast::TypeName::Int8;

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
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::typename;
use pg_ast::Typecast;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Cast;
