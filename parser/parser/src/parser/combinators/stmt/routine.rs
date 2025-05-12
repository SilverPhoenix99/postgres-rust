pub(super) fn routine() -> impl Combinator<Output = FunctionWithArgs> {

    /*
        ROUTINE function_with_argtypes
    */

    Routine
        .and_right(function_with_argtypes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_routine() {
        test_parser!(
            source = "routine foo",
            parser = routine(),
            expected = FunctionWithArgs::new(vec!["foo".into()], None)
        )
    }
}

use crate::parser::ast_node::FunctionWithArgs;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::function_with_argtypes;
use postgres_parser_lexer::Keyword::Routine;
