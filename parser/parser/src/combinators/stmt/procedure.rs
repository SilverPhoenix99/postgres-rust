pub(super) fn procedure(stream: &mut TokenStream) -> Result<FunctionWithArgs> {

    /*
        PROCEDURE function_with_argtypes
    */

    seq!(stream => Procedure, function_with_argtypes)
        .map(|(_, signature)| signature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

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
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::FunctionWithArgs;
use pg_lexer::Keyword::Procedure;
