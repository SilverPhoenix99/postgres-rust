pub(super) fn function(stream: &mut TokenStream) -> scan::Result<FunctionWithArgs> {

    /*
        FUNCTION function_with_argtypes
    */

    let (_, signature) = seq!(Function, function_with_argtypes)
        .parse(stream)?;

    Ok(signature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_function() {
        test_parser!(
            source = "function foo",
            parser = function,
            expected = FunctionWithArgs::new(vec!["foo".into()], None)
        )
    }
}

use crate::combinators::function_with_argtypes;
use pg_ast::FunctionWithArgs;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Function;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
