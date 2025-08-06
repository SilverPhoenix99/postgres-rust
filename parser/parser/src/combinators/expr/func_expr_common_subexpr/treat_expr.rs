pub(super) fn treat_expr(stream: &mut TokenStream) -> scan::Result<SqlFunction> {

    /*
        TREAT '(' a_expr AS Typename ')'

        Converts the expression of a particular type to a target type,
        which is defined to be a subtype of the original expression.
        In SQL99, this is intended for use with structured UDTs,
        but let's make this a generally useful form allowing stronger
        coercions than are handled by implicit casting.
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (expr, _, typename)) = seq!(
        skip(1),
        paren!(seq!(a_expr, As, typename))
    ).parse(stream)?;

    let cast = TypecastExpr::new(expr, typename);
    let expr = Treat(cast);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::IntegerConst,
        pg_ast::TypeName::Int4,
    };

    #[test_case("treat(123 as int)" => Ok(
        Treat(
            TypecastExpr::new(
                IntegerConst(123),
                Int4
            )
        )
    ))]
    fn test_treat_expr(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, treat_expr)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::combinators::typename;
use crate::stream::TokenStream;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::Treat;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword::As;
use pg_parser_core::scan;
