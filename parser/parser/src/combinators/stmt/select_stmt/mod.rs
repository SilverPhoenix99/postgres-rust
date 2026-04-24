mod for_locking_clause;

/// Alias: `SelectStmt`
pub(in crate::combinators) fn select_stmt(ctx: &mut ParserContext) -> scan::Result<SelectStmt> {

    /*
        ( with_clause )? direct_select_stmt
    */

    todo!()
}

/// `SelectStmt` without CTE
pub(in crate::combinators) fn direct_select_stmt(ctx: &mut ParserContext) -> scan::Result<SelectStmt> {

    /*
        select_clause ( sort_clause )? ( select_suffix )?
    */

    todo!()
}

pg_basics::reexport! {
    having_clause,
}

pub(in crate::combinators) fn is_select_stmt(ctx: &mut ParserContext) -> bool {

    /*
        - WITH | SELECT | TABLE are Reserved keywords, so they don't conflict on many use cases.
        - VALUES is ColumnName, so it needs to check for '('.
    */

    matches!(ctx.stream_mut().peek_n::<2>(),
        Ok(
            [Keyword(With | Select | Table), _]
            | [Keyword(Values), Operator(OpenParenthesis)]
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("select 1" => ignore["select_stmt not implemented yet"] matches Ok(_))]
    fn test_select_stmt(source: &str) -> scan::Result<SelectStmt> {
        test_parser!(source, select_stmt)
    }
}

use crate::context::ParserContext;
use pg_ast::SelectStmt;
use pg_lexer::Keyword::Select;
use pg_lexer::Keyword::Table;
use pg_lexer::Keyword::Values;
use pg_lexer::Keyword::With;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
