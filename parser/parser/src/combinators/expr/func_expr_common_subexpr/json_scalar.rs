pub(super) fn json_scalar(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        JSON_SCALAR '(' a_expr ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, arg) = seq!(skip(1), paren!(a_expr))
        .parse(stream)?;

    let arg = Box::new(arg);
    Ok(JsonScalar(arg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;

    #[test_case("json_scalar(1)" => Ok(
        JsonScalar(Box::new(
            IntegerConst(1)
        ))
    ))]
    fn test_json_scalar(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, json_scalar)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_ast::ExprNode::JsonScalar;
