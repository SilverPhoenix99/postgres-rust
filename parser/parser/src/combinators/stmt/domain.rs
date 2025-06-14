pub(super) fn domain() -> impl Combinator<Output=Type> {

    /*
        DOMAIN Typename
    */

    Domain
        .and_right(typename())
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
            parser = domain(),
            expected = Int4.into()
        )
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::typename;
use pg_ast::Type;
use pg_lexer::Keyword::Domain;
