pub(super) fn domain(stream: &mut TokenStream) -> scan::Result<Type> {

    /*
        DOMAIN Typename
    */

    let (_, typ) = seq!(Domain, typename).parse(stream)?;

    Ok(typ)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::TypeName::Int4;
    use pg_combinators::test_parser;

    #[test]
    fn test_domain() {
        test_parser!(
            source = "domain int",
            parser = domain,
            expected = Int4
        )
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::typename;
use pg_ast::Type;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Domain;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
