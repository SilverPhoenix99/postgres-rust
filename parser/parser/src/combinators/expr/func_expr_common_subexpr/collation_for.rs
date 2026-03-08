pub(super) fn collation_for(ctx: &mut ParserContext) -> scan::Result<SqlFunction> {

    /*
        COLLATION FOR '(' a_expr ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, expr) = seq!(skip(2), paren!(a_expr))
        .parse(ctx)?;

    Ok(CollationFor(expr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::StringConst;
    use test_case::test_case;

    #[test_case("collation for ('foo')" => Ok(
        CollationFor(
            StringConst("foo".into())
        )
    ))]
    fn test_collation_for(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, collation_for)
    }
}

use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::CollationFor;
use pg_parser_core::scan;
