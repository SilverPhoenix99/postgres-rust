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
    use pg_combinators::test_parser;

    #[test]
    fn test_show_stmt() {
        test_parser!(
            source = "show time zone",
            parser = show_stmt,
            expected = VariableTarget::TimeZone
        )
    }
}

use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_generic_set_ast::VariableTarget;
use pg_generic_set_combinators::variable_target;
use pg_lexer::Keyword::Show;
use pg_parser_core::scan;
