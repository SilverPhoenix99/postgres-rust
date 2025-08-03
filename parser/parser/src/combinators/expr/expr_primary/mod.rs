mod func_application;

pg_basics::reexport! {
    case_expr,
    explicit_row,
    func_expr,
    grouping_func,
    param_expr,
    prefixed_expr,
}

/// Alias: `c_expr`
pub(in crate::combinators) fn expr_primary(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    alt!(
        param_expr,
        expr_const,
        case_expr.map(From::from),
        func_expr,
        explicit_row,
        grouping_func,

        // ❗ Must be after most other productions,
        // due to conflicts with the 1st keyword.
        prefixed_expr,
    ).parse(stream)
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

use crate::combinators::expr::expr_const;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
