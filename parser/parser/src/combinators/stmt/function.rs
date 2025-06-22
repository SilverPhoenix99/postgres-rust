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
    use crate::tests::test_parser;

    #[test]
    fn test_function() {
        test_parser!(
            source = "function foo",
            parser = function(),
            expected = FunctionWithArgs::new(vec!["foo".into()], None)
        )
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::function_with_argtypes;
use pg_ast::FunctionWithArgs;
use pg_lexer::Keyword::Function;
