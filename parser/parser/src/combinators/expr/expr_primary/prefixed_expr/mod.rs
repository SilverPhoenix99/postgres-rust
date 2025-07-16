mod attr_tail;
mod identifier_prefixed_expr;
mod tailed_expr;
mod type_func_name_prefixed_expr;

pub(super) fn prefixed_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    or((
        identifier_prefixed_expr,
        type_func_name_prefixed_expr
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::stream;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ColumnRef,
        pg_ast::ExprNode::{IntegerConst, StringConst},
        pg_ast::FuncArgExpr,
        pg_ast::FuncArgsKind,
        pg_ast::FuncArgsOrder,
        pg_ast::FuncCall,
        pg_ast::OverClause,
        pg_ast::SortBy,
        pg_ast::TypeName,
        pg_ast::TypecastExpr,
        pg_basics::Location,
    };

    #[test_case("foo.bar")] // identifier_prefixed_expr
    #[test_case("inner()")] // type_func_name_prefixed_expr
    fn test_prefixed_expr(source: &str) {
        let mut stream = stream(source);
        let actual = prefixed_expr(&mut stream);

        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
    }
}

use self::{
    attr_tail::*,
    identifier_prefixed_expr::*,
    tailed_expr::*,
    type_func_name_prefixed_expr::*,
};
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
