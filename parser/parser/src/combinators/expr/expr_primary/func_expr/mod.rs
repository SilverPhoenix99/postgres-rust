mod extract_list;
mod filter_clause;
mod func_expr_common_subexpr;
mod over_clause;
mod within_group_clause;

pub(super) use {
    filter_clause::*,
    over_clause::*,
    within_group_clause::*,
};

pub(super) fn func_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
          func_expr_common_subexpr
        | json_aggregate_func filter_clause over_clause
    */

    func_expr_common_subexpr(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_matrix;

    // These only quickly check that statements aren't missing:
    #[test_matrix(["collation for (5)"] => matches Ok(_))]
    fn test_func_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, func_expr)
    }
}

use self::func_expr_common_subexpr::func_expr_common_subexpr;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
