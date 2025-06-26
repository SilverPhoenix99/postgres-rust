pub(super) fn expr_list_paren() -> impl Combinator<Output = Vec<ExprNode>> {

    /*
        '(' expr_list ')'
    */

    parser(|stream| between!(paren : stream =>
        expr_list().parse(stream)
    ))
}

use crate::combinators::expr_list;
use crate::combinators::foundation::{between, parser, Combinator};
use pg_ast::ExprNode;
