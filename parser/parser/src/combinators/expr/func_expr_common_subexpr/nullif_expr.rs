pub(super) fn nullif_expr(ctx: &mut ParserContext) -> scan::Result<SqlFunction> {

    /*
        NULLIF '(' a_expr ',' a_expr ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (left, _, right)) = seq!(
        skip(1),
        paren!(seq!(a_expr, Comma, a_expr))
    ).parse(ctx)?;

    Ok(NullIf(left, right))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{NullConst, StringConst};
    use pg_ast::SqlFunction;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("nullif(null, 'foo')" => Ok(
        NullIf(
            NullConst,
            StringConst("foo".into())
        )
    ))]
    fn test_nullif_expr(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, nullif_expr)
    }
}

use crate::combinators::expr::a_expr;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::NullIf;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
