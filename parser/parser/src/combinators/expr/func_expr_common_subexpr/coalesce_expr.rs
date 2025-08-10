pub(super) fn coalesce_expr(ctx: &mut ParserContext) -> scan::Result<SqlFunction> {

    /*
        COALESCE '(' expr_list ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, args) = seq!(skip(1), paren!(expr_list))
        .parse(ctx)?;

    Ok(Coalesce(args))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::StringConst;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("coalesce('foo', 'bar')" => Ok(
        Coalesce(vec![
            StringConst("foo".into()),
            StringConst("bar".into())
        ])
    ))]
    fn test_coalesce_expr(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, coalesce_expr)
    }
}

use crate::combinators::expr_list::expr_list;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::Coalesce;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
