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
    use crate::parser::ast_node::VariableTarget;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_show_stmt() {
        let source = "show time zone";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(VariableTarget::TimeZone), show_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Show;
use crate::parser::ast_node::VariableTarget;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::stmt::variable_target;
