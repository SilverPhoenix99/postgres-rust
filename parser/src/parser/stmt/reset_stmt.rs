/// Alias: `VariableResetStmt`
pub(in crate::parser) fn reset_stmt() -> impl Combinator<Output = VariableTarget> {

    /*
        RESET variable_target
    */

    Reset.and_right(variable_target())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::VariableTarget;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_variable_reset_stmt() {
        let source = "reset transaction isolation level";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(VariableTarget::TransactionIsolation), reset_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Reset;
use crate::parser::ast_node::VariableTarget;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::stmt::variable_target::variable_target;
