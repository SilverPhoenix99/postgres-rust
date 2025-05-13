pub(super) fn function() -> impl Combinator<Output = FunctionWithArgs> {

    /*
        FUNCTION function_with_argtypes
    */

    Function
        .and_right(function_with_argtypes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_function() {
        test_parser!(
            source = "function foo",
            parser = function(),
            expected = FunctionWithArgs::new(vec!["foo".into()], None)
        )
    }
}

use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::function_with_argtypes;
use postgres_parser_ast::FunctionWithArgs;
use postgres_parser_lexer::Keyword::Function;
