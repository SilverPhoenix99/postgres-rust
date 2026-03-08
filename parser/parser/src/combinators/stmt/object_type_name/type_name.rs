pub(in crate::combinators::stmt) fn type_name(ctx: &mut ParserContext) -> scan::Result<Type> {

    /*
        TYPE Typename
    */

    let (_, typ) = seq!(Kw::Type, typename).parse(ctx)?;

    Ok(typ)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::TypeName::Int4;

    #[test]
    fn test_type() {
        test_parser!(
            source = "type int",
            parser = type_name,
            expected = Int4
        )
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::typename;
use crate::seq;
use crate::ParserContext;
use pg_ast::Type;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
