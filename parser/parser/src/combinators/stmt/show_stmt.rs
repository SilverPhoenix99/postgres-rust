/// Alias: `VariableShowStmt`
pub(super) fn show_stmt(ctx: &mut ParserContext) -> scan::Result<VariableTarget> {

    /*
        SHOW variable_target
    */

    let (_, target) = seq!(Show, variable_target)
        .parse(ctx)?;

    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_show_stmt() {
        test_parser!(
            source = "show time zone",
            parser = show_stmt,
            expected = VariableTarget::TimeZone
        )
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::variable_target;
use crate::seq;
use crate::ParserContext;
use pg_ast::VariableTarget;
use pg_lexer::Keyword::Show;
use pg_parser_core::scan;
