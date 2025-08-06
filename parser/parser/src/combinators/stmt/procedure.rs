pub(super) fn procedure(stream: &mut TokenStream) -> scan::Result<FunctionWithArgs> {

    /*
        PROCEDURE function_with_argtypes
    */

    let (_, signature) = seq!(Procedure, function_with_argtypes)
        .parse(stream)?;

    Ok(signature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_procedure() {
        test_parser!(
            source = "procedure foo",
            parser = procedure,
            expected = FunctionWithArgs::new(vec!["foo".into()], None)
        )
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::function_with_argtypes;
use pg_ast::FunctionWithArgs;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Procedure;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
