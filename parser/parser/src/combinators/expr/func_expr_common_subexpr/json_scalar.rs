pub(super) fn json_scalar(stream: &mut TokenStream) -> scan::Result<SqlFunction> {

    /*
        JSON_SCALAR '(' a_expr ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, arg) = seq!(skip(1), paren!(a_expr))
        .parse(stream)?;

    Ok(JsonScalar(arg))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("json_scalar(1)" => Ok(
        JsonScalar(IntegerConst(1))
    ))]
    fn test_json_scalar(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, json_scalar)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::JsonScalar;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
