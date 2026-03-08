/// Alias: `VariableResetStmt`
pub(super) fn reset_stmt(ctx: &mut ParserContext) -> scan::Result<VariableTarget> {

    /*
        RESET variable_target
    */

    let (_, target) = seq!(Reset, variable_target)
        .parse(ctx)?;

    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_variable_reset_stmt() {
        test_parser!(
            source = "reset transaction isolation level",
            parser = reset_stmt,
            expected = VariableTarget::TransactionIsolation
        )
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::variable_target;
use crate::seq;
use crate::ParserContext;
use pg_ast::VariableTarget;
use pg_lexer::Keyword::Reset;
use pg_parser_core::scan;
