pub(super) fn nullif_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        NULLIF '(' a_expr ',' a_expr ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (left, _, right)) = seq!(
        skip(1),
        paren!(seq!(a_expr, Comma, a_expr))
    ).parse(stream)?;

    let operands = Box::new((left, right));
    Ok(NullIf(operands))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{NullConst, StringConst};
    use test_case::test_case;

    #[test_case("nullif(null, 'foo')" => Ok(
        NullIf(Box::new((
            NullConst,
            StringConst("foo".into())
        )))
    ))]
    fn test_nullif_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, nullif_expr)
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
use pg_ast::ExprNode::NullIf;
use pg_lexer::OperatorKind::Comma;
