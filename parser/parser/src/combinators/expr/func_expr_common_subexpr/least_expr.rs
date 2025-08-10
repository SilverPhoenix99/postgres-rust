pub(super) fn least_expr(ctx: &mut ParserContext) -> scan::Result<SqlFunction> {

    /*
        LEAST '(' expr_list ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let expr_list = ctx.expr_list();
    let (_, args) = seq!(skip(1), paren!(expr_list))
        .parse(ctx)?;

    Ok(Least(args))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::expr_list;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::IntegerConst,
        scan::Error::NoMatch,
    };

    #[test_case("least(1, 2)" => Ok(
        Least(vec![
            IntegerConst(1),
            IntegerConst(2)
        ])
    ))]
    fn test_greatest_expr(source: &str) -> scan::Result<SqlFunction> {
        let mut ctx = ParserContext::new(source, expr_list);
        least_expr(&mut ctx)
    }
}

use pg_ast::SqlFunction;
use pg_ast::SqlFunction::Least;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_parser_core::scan;
