pub(super) fn type_name(stream: &mut TokenStream) -> scan::Result<Type> {

    let (_, typ) = seq!(Kw::Type, typename).parse(stream)?;

    Ok(typ)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::TypeName::Int4;
    use pg_combinators::test_parser;

    #[test]
    fn test_type() {
        test_parser!(
            source = "type int",
            parser = type_name,
            expected = Int4
        )
    }
}

use crate::combinators::typename;
use pg_ast::Type;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
