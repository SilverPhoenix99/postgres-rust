/// Alias: `CheckPointStmt`
pub(super) fn check_point_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
          CHECKPOINT
        | CHECKPOINT '(' utility_option_list ')'
    */

    let (_, options) = (
        Checkpoint,
        utility_options.optional()
    ).parse(stream)?;

    Ok(CheckPointStmt(options))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::UtilityOptionName::Analyze;
    use test_case::test_case;

    #[test_case("checkpoint", CheckPointStmt(None))]
    #[test_case("checkpoint(analyze)",
        CheckPointStmt(Some(vec![Analyze.into()]))
    )]
    fn test_check_point_stmt(source: &str, expected: RawStmt) {
        test_parser!(source, check_point_stmt, expected)
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::stmt::utility_options;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_ast::RawStmt::CheckPointStmt;
use pg_lexer::Keyword::Checkpoint;
