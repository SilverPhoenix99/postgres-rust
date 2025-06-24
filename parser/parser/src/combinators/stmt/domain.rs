pub(super) fn domain(stream: &mut TokenStream) -> Result<Type> {

    /*
        DOMAIN Typename
    */

    seq!(stream => Domain, typename)
        .map(|(_, typ)| typ)
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

use crate::combinators::foundation::seq;
use crate::combinators::typename;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::Type;
use pg_lexer::Keyword::Domain;
