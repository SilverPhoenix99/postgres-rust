pub(super) fn type_name(stream: &mut TokenStream) -> scan::Result<Type> {

    let (_, typ) = seq!(stream => Kw::Type, typename)?;

    Ok(typ)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
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

use crate::combinators::foundation::seq;
use crate::combinators::typename;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::Type;
use pg_lexer::Keyword as Kw;
