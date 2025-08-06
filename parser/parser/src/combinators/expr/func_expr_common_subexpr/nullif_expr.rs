pub(super) fn nullif_expr(stream: &mut TokenStream) -> scan::Result<SqlFunction> {

    /*
        NULLIF '(' a_expr ',' a_expr ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (left, _, right)) = seq!(
        skip(1),
        paren!(seq!(a_expr, Comma, a_expr))
    ).parse(stream)?;

    Ok(NullIf(left, right))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{NullConst, StringConst};
    use pg_ast::SqlFunction;
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
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::NullIf;
use pg_combinators::Combinator;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
