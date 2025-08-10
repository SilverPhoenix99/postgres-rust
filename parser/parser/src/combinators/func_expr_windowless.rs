pub(super) fn func_expr_windowless(ctx: &mut ParserContext) -> scan::Result<FuncExprWindowless> {

    /*
          func_expr_common_subexpr
        | json_aggregate_func
        | func_application
    */

    alt!(
        func_expr_common_subexpr.map(From::from),
        json_aggregate_func.map(From::from),
        func_application.map(From::from),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_matrix;

    // These only quickly check that statements aren't missing
    #[test_matrix(
        [
            "collation for(1)",
            "json_arrayagg(1)",
            "foo(1)",
        ]
        => matches Ok(_)
    )]
    fn test_func_expr_windowless(source: &str) -> scan::Result<FuncExprWindowless> {
        test_parser!(source, func_expr_windowless)
    }
}

use crate::combinators::expr::func_expr_common_subexpr;
use crate::combinators::func_application;
use crate::combinators::json_aggregate_func;
use pg_ast::FuncExprWindowless;
use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
