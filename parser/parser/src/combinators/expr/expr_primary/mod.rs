pg_basics::reexport! {
    array_expr,
    case_expr,
    exists_expr,
    expr_primary_paren,
    func_expr,
    grouping_func,
    param_expr,
    prefixed_expr_const,
}

/// Alias: `c_expr`
pub(in crate::combinators) fn expr_primary(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
          PARAM ( indirection )?
        | AexprConst
        | CASE ... WHEN ... END
        | func_expr
        | ROW '(' ... ')'
        | GROUPING '(' ... ')'
        | ARRAY '[' ... ']'
        | EXISTS '(' SelectStmt ')'
        | '(' ... ')'
    */

    alt!(
        param_expr,
        expr_const,
        case_expr.map(From::from),
        func_expr,
        explicit_row.map(Row),
        grouping_func,
        array_expr,
        exists_expr,
        expr_primary_paren,

        // ❗ Must be after most other productions,
        // due to conflicts with the 1st keyword.
        prefixed_expr_const,
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    // These only quickly check that statements aren't missing:
    #[test_matrix(
        [
            "$3",                     // param_expr
            "true",                   // expr_const
            "case when 1 then 2 end", // case_expr
            "user",                   // func_expr
            "row()",                  // explicit_row
            "grouping(1)",            // grouping_func
            "current_schema",         // prefix_expr
            "(1, 2)",                 // expr_primary_paren
            "array[1, 2]",            // array_expr
        ]
        => matches Ok(_)
    )]
    fn test_expr_primary(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, expr_primary)
    }

    #[test_matrix("exists (select 1)" => ignore["select_stmt not implemented yet"] matches Ok(_))]
    fn test_expr_primary_exists_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, expr_primary)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::expr::explicit_row;
use crate::combinators::expr::expr_const;
use crate::ParserContext;
use pg_ast::ExprNode;
use pg_ast::ExprNode::Row;
use pg_parser_core::scan;
