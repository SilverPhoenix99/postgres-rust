mod case_expr;
mod cast_expr;
mod explicit_row;
mod func_application;
mod func_expr;
mod grouping_func;
mod param_expr;
mod prefixed_expr;

/// Alias: `c_expr`
pub(super) fn expr_primary(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    or((
        param_expr,
        expr_const,
        case_expr.map(From::from),
        func_expr,
        explicit_row,
        grouping_func,

        // ❗ Must be after most other productions,
        // due to conflicts with the 1st keyword.
        prefixed_expr,
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_matrix;

    // These only quickly check that statements aren't missing:
    #[test_matrix(
        [
            "$3",                     // param_expr
            "true",                   // expr_const
            "case when 1 then 2 end", // case_expr
            "user",                   // func_expr
            "row()",                  // explicit_row
            "grouping(1)",            // explicit_row
            "current_schema",         // prefix_expr
        ]
        => matches Ok(_)
    )]
    fn test_expr_primary(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, expr_primary)
    }
}

use self::{
    case_expr::case_expr,
    cast_expr::cast_expr,
    explicit_row::explicit_row,
    func_expr::func_expr,
    grouping_func::grouping_func,
    param_expr::param_expr,
    prefixed_expr::prefixed_expr,
};
use crate::combinators::expr::expr_const;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
