/// Alias: `CheckPointStmt`
pub(super) fn check_point_stmt(ctx: &mut ParserContext) -> scan::Result<Option<Vec<UtilityOption>>> {

    /*
        CHECKPOINT ( utility_options )?
    */

    let (_, options) = seq!(
        Checkpoint,
        utility_options.optional()
    ).parse(ctx)?;

    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::UtilityOptionName::Analyze;
    use test_case::test_case;

    #[test_case("checkpoint" => Ok(None))]
    #[test_case("checkpoint(analyze)" => Ok(Some(vec![Analyze.into()])))]
    fn test_check_point_stmt(source: &str) -> scan::Result<Option<Vec<UtilityOption>>> {
        test_parser!(source, check_point_stmt)
    }
}

use super::utility_options;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::UtilityOption;
use pg_lexer::Keyword::Checkpoint;
use pg_parser_core::scan;
