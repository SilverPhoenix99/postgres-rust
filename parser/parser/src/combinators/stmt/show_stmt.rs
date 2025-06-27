/// Alias: `VariableShowStmt`
pub(super) fn show_stmt(stream: &mut TokenStream) -> scan::Result<VariableTarget> {

    /*
        SHOW variable_target
    */

    let (_, target) = seq!(stream => Show, variable_target)?;

    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_show_stmt() {
        test_parser!(
            source = "show time zone",
            parser = show_stmt,
            expected = VariableTarget::TimeZone
        )
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::stmt::variable_target;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::VariableTarget;
use pg_lexer::Keyword::Show;
