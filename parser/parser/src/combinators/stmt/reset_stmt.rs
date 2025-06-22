/// Alias: `VariableResetStmt`
pub(super) fn reset_stmt() -> impl Combinator<Output = VariableTarget> {

    /*
        RESET variable_target
    */

    Reset.and_right(variable_target())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_variable_reset_stmt() {
        let source = "reset transaction isolation level";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(VariableTarget::TransactionIsolation), reset_stmt().parse(&mut stream));
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::stmt::variable_target;
use pg_ast::VariableTarget;
use pg_lexer::Keyword::Reset;
