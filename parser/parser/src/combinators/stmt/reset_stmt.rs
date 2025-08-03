/// Alias: `VariableResetStmt`
pub(super) fn reset_stmt(stream: &mut TokenStream) -> scan::Result<VariableTarget> {

    /*
        RESET variable_target
    */

    let (_, target) = seq!(Reset, variable_target)
        .parse(stream)?;

    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_variable_reset_stmt() {
        test_parser!(
            source = "reset transaction isolation level",
            parser = reset_stmt,
            expected = VariableTarget::TransactionIsolation
        )
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::stmt::variable_target;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::VariableTarget;
use pg_lexer::Keyword::Reset;
