pub(super) fn domain(stream: &mut TokenStream) -> scan::Result<Type> {

    /*
        DOMAIN Typename
    */

    let (_, typ) = (Domain, typename).parse(stream)?;

    Ok(typ)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::TypeName::Int4;

    #[test]
    fn test_domain() {
        test_parser!(
            source = "domain int",
            parser = domain,
            expected = Int4
        )
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::typename;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::Type;
use pg_lexer::Keyword::Domain;
