pub(super) fn procedure() -> impl Combinator<Output = FunctionWithArgs> {

    /*
        PROCEDURE function_with_argtypes
    */

    Procedure
        .and_right(function_with_argtypes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_procedure() {
        test_parser!(
            source = "procedure foo",
            parser = procedure(),
            expected = FunctionWithArgs::new(vec!["foo".into()], None)
        )
    }
}

use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::function_with_argtypes;
use postgres_parser_ast::FunctionWithArgs;
use postgres_parser_lexer::Keyword::Procedure;
