pub(in crate::combinators::stmt) fn domain(ctx: &mut ParserContext) -> scan::Result<Type> {

    /*
        DOMAIN Typename
    */

    let (_, typ) = seq!(Domain, typename).parse(ctx)?;

    Ok(typ)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
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

use crate::combinators::core::Combinator;
use crate::combinators::typename;
use crate::seq;
use crate::ParserContext;
use pg_ast::Type;
use pg_lexer::Keyword::Domain;
use pg_parser_core::scan;
