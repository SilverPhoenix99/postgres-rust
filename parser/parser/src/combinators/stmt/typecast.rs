pub(super) fn typecast(stream: &mut TokenStream) -> scan::Result<Typecast> {

    /*
        CAST '(' Typename AS Typename ')'
    */

    let (_, (from_type, _, to_type)) = seq!(
        Cast,
        paren!(seq!(typename, As, typename))
    ).parse(stream)?;

    Ok(Typecast::new(from_type, to_type))
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
            parser = typecast,
            expected = Typecast::new(Int4, Int8)
        )
    }
}

use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::typename;
use crate::stream::TokenStream;
use pg_ast::Typecast;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Cast;
use pg_parser_core::scan;
