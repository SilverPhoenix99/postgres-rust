pub(super) fn least_expr(stream: &mut TokenStream) -> scan::Result<SqlFunction> {

    /*
        LEAST '(' expr_list ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, args) = seq!(skip(1), paren!(expr_list))
        .parse(stream)?;

    Ok(Least(args))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
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
        test_parser!(source, least_expr)
    }
}

use crate::combinators::expr_list::expr_list;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::Least;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
