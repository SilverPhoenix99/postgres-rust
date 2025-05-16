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
    use crate::tests::test_parser;

    #[test]
    fn test_routine() {
        test_parser!(
            source = "routine foo",
            parser = routine(),
            expected = FunctionWithArgs::new(vec!["foo".into()], None)
        )
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::function_with_argtypes;
use pg_ast::FunctionWithArgs;
use pg_lexer::Keyword::Routine;
