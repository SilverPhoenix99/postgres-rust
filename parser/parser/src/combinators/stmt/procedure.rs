pub(super) fn procedure(stream: &mut TokenStream) -> scan::Result<FunctionWithArgs> {

    /*
        PROCEDURE function_with_argtypes
    */

    let (_, signature) = seq!(stream => Procedure, function_with_argtypes)?;

    Ok(signature)
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
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::FunctionWithArgs;
use pg_lexer::Keyword::Procedure;
