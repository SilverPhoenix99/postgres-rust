pub(super) fn func_expr_windowless(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
          func_expr_common_subexpr
        | json_aggregate_func
        | func_application
    */

    alt!(
        func_expr_common_subexpr,
        json_aggregate_func.map(From::from),
        func_application.map(From::from),
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
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
    fn test_func_expr_windowless(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, func_expr_windowless)
    }
}

use crate::combinators::expr::func_expr_common_subexpr;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::Combinator;
use crate::combinators::func_application;
use crate::combinators::json_aggregate_func;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
