/// Alias: `CheckPointStmt`
pub(super) fn check_point_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        CHECKPOINT ( utility_options )?
    */

    let (_, options) = seq!(
        Checkpoint,
        utility_options.optional()
    ).parse(ctx)?;

    Ok(CheckPointStmt(options))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    #[allow(unused_imports)]
    use pg_utility_option_ast::UtilityOptionName::Analyze;
    use test_case::test_case;

    #[test_case("checkpoint", CheckPointStmt(None))]
    #[test_case("checkpoint(analyze)",
        CheckPointStmt(Some(vec![Analyze.into()]))
    )]
    fn test_check_point_stmt(source: &str, expected: RawStmt) {
        test_parser!(source, check_point_stmt, expected)
    }
}

use pg_ast::RawStmt;
use pg_ast::RawStmt::CheckPointStmt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Checkpoint;
use pg_parser_core::scan;
use pg_utility_option_combinators::utility_options;
