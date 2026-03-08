pub(in crate::combinators::stmt) fn function(ctx: &mut ParserContext) -> scan::Result<FunctionWithArgs> {

    /*
        FUNCTION function_with_argtypes
    */

    let (_, signature) = seq!(Function, function_with_argtypes)
        .parse(ctx)?;

    Ok(signature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_function() {
        test_parser!(
            source = "function foo",
            parser = function,
            expected = FunctionWithArgs::new(vec!["foo".into()], None)
        )
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::function_with_argtypes;
use crate::seq;
use crate::ParserContext;
use pg_ast::FunctionWithArgs;
use pg_lexer::Keyword::Function;
use pg_parser_core::scan;
