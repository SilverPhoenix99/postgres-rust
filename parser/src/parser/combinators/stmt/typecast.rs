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
    use crate::parser::ast_node::TypeName::Int4;
    use crate::parser::ast_node::TypeName::Int8;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_typecast() {
        test_parser!(
            source = "cast (int as bigint)",
            parser = typecast(),
            expected = Typecast::new(Int4, Int8)
        )
    }
}

use crate::lexer::Keyword::As;
use crate::lexer::Keyword::Cast;
use crate::parser::ast_node::Typecast;
use crate::parser::combinators::between_paren;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::typename;
