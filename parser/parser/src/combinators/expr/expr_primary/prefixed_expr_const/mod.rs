pg_basics::reexport! {
    attr_tail,
    identifier_prefixed_expr,
    tailed_expr,
    type_func_name_prefixed_expr,
}

pub(super) fn prefixed_expr_const(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
          func_name                              => columnref
        | func_name SCONST                       => AexprConst
        | func_name '(' func_arg_list ')' SCONST => AexprConst
        | func_application func_args_tail        => func_expr
    */

    // ❗ This function replaces `func_application`, to workaround conflicts with `expr_const` and `expr_primary`.

    alt!(
        identifier_prefixed_expr,
        type_func_name_prefixed_expr
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_matrix;

    // These only quickly check that statements aren't missing:
    #[test_matrix(
        [
            "foo.bar", // identifier_prefixed_expr
            "inner()", // type_func_name_prefixed_expr
        ]
        => matches Ok(_)
    )]
    fn test_prefixed_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, prefixed_expr_const)
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
