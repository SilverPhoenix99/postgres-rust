mod case_expr;
mod cast_expr;
mod func_expr;
mod param_expr;

/// Alias: `c_expr`
pub(super) fn expr_primary() -> impl Combinator<Output = ExprNode> {
    match_first! {
        param_expr(),
        expr_const(),
        func_expr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("$3")]
    #[test_case("true")]
    #[test_case("user")]
    fn test_expr_primary(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = expr_primary().parse(&mut stream);

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
    func_expr::func_expr,
    param_expr::param_expr,
};
use crate::parser::ast_node::ExprNode;
use crate::parser::combinators::expr::expr_const;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;

