pub(super) fn collation_for(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        COLLATION FOR '(' a_expr ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, expr) = seq!(skip(2), paren!(a_expr))
        .parse(stream)?;

    let expr = Box::new(expr);
    Ok(CollationForFunc(expr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::StringConst;
    use test_case::test_case;

    #[test_case("collation for ('foo')" => Ok(
        CollationForFunc(
            Box::new(StringConst("foo".into()))
        )
    ))]
    fn test_collation_for(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, collation_for)
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
use pg_ast::ExprNode::CollationForFunc;
