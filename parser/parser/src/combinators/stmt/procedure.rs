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
    use crate::tests::test_parser;

    #[test]
    fn test_procedure() {
        test_parser!(
            source = "procedure foo",
            parser = procedure(),
            expected = FunctionWithArgs::new(vec!["foo".into()], None)
        )
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::function_with_argtypes;
use pg_ast::FunctionWithArgs;
use pg_lexer::Keyword::Procedure;
