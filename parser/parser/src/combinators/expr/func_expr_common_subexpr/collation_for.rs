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
    #[allow(unused_imports)]
    use pg_ast::ExprNode::StringConst;
    use pg_combinators::test_parser;
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

use crate::combinators::expr::a_expr;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::CollationFor;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_parser_core::scan;
