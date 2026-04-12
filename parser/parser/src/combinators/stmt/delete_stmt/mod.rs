pg_basics::reexport! {
    using_clause,
}

/// Alias: `DeleteStmt`
pub(super) fn delete_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        ( with_clause )? direct_delete_stmt
    */

    todo!()
}

/// `DeleteStmt` without CTE
pub(super) fn direct_delete_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          DELETE FROM (
              relation_expr_opt_alias using_clause where_or_current_clause returning_clause
            | relation_expr for_portion_of_clause for_portion_of_opt_alias using_clause where_or_current_clause returning_clause
        )
    */

    todo!()
}

use crate::context::ParserContext;
use pg_ast::RawStmt;
use pg_parser_core::scan;
