pub(super) fn routine(stream: &mut TokenStream) -> scan::Result<FunctionWithArgs> {

    /*
        ROUTINE function_with_argtypes
    */

    let (_, signature) = (Routine, function_with_argtypes)
        .parse(stream)?;

    Ok(signature)
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

use crate::combinators::foundation::Combinator;
use crate::combinators::function_with_argtypes;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::FunctionWithArgs;
use pg_lexer::Keyword::Routine;
