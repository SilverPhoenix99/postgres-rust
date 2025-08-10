pub(in crate::combinators::stmt) fn routine(ctx: &mut ParserContext) -> scan::Result<FunctionWithArgs> {

    /*
        ROUTINE function_with_argtypes
    */

    let (_, signature) = seq!(Routine, function_with_argtypes)
        .parse(ctx)?;

    Ok(signature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_routine() {
        test_parser!(
            source = "routine foo",
            parser = routine,
            expected = FunctionWithArgs::new(vec!["foo".into()], None)
        )
    }
}

use crate::combinators::function_with_argtypes;
use pg_ast::FunctionWithArgs;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Routine;
use pg_parser_core::scan;
