/// Alias: `VariableShowStmt`
pub(super) fn show_stmt() -> impl Combinator<Output = VariableTarget> {

    /*
        SHOW variable_target
    */

    Show.and_right(variable_target())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_show_stmt() {
        let source = "show time zone";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(VariableTarget::TimeZone), show_stmt().parse(&mut stream));
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::stmt::variable_target;
use pg_ast::VariableTarget;
use pg_lexer::Keyword::Show;
