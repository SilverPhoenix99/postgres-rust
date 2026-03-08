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
    use test_case::test_case;

    #[test_case("coalesce('foo', 'bar')" => Ok(
        Coalesce(vec![
            StringConst("foo".into()),
            StringConst("bar".into())
        ])
    ))]
    fn test_coalesce_expr(source: &str) -> scan::Result<SqlFunction> {
        let mut ctx = ParserContext::new(source);
        coalesce_expr(&mut ctx)
    }
}

use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::expr_list;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::Coalesce;
use pg_parser_core::scan;
