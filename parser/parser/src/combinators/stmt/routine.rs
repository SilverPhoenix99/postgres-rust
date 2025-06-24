pub(super) fn routine(stream: &mut TokenStream) -> Result<FunctionWithArgs> {

    /*
        ROUTINE function_with_argtypes
    */

    seq!(stream => Routine, function_with_argtypes)
        .map(|(_, signature)| signature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_routine() {
        test_parser!(
            source = "routine foo",
            parser = routine,
            expected = FunctionWithArgs::new(vec!["foo".into()], None)
        )
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::function_with_argtypes;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::FunctionWithArgs;
use pg_lexer::Keyword::Routine;
