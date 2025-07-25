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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("$3")] // param_expr
    #[test_case("true")] // expr_const
    #[test_case("user")] // func_expr
    #[test_case("row()")] // explicit_row
    #[test_case("grouping(1)")] // explicit_row
    #[test_case("current_schema")] // prefix_expr
    fn test_expr_primary(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = expr_primary(&mut stream);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
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
