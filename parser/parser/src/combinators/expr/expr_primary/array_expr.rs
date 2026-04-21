pub(super) fn array_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        ARRAY (
              '[' ( array_expr_list | expr_list )? ']'
            | '(' select_stmt ')'
        )
    */

    let (_, expr) = seq!(
        Kw::Array,
        alt!(
            array_expr_inner.map(Array),
            paren!(select_stmt).map(From::from)
        )
    ).parse(ctx)?;

    Ok(expr)
}

/// The `Option` result does not come from an absence of value.
/// It returns `None` for the expression `[]`.
fn array_expr_inner(ctx: &mut ParserContext) -> scan::Result<Option<Vec<ExprNode>>> {

    /*
        '[' ( array_expr_list | expr_list )? ']'
    */

    brackets!(
        alt!(array_expr_list, expr_list).optional()
    ).parse(ctx)
}

fn array_expr_list(ctx: &mut ParserContext) -> scan::Result<Vec<ExprNode>> {

    /*
        array_expr_inner ( ',' array_expr_inner )*
    */

    many!(
        sep = Comma,
        array_expr_inner.map(Array)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use test_case::test_case;

    #[test_case("array[]" => Ok(Array(None)); "empty array")]
    #[test_case("array[1]" => Ok(Array(Some(vec![IntegerConst(1)]))))]
    #[test_case("array(select1)" => ignore["select_stmt not implemented yet"] matches Ok(_))]
    fn test_array_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, array_expr)
    }

    #[test_case("[]" => Ok(None); "empty array")]
    #[test_case("['foo', 'bar']" => Ok(Some(vec![
        StringConst("foo".into()),
        StringConst("bar".into())
    ])))]
    #[test_case("[['baz'],[1]]" => Ok(Some(vec![
        Array(Some(vec![StringConst("baz".into())])),
        Array(Some(vec![IntegerConst(1)]))
    ])))]
    fn test_array_expr_inner(source: &str) -> scan::Result<Option<Vec<ExprNode>>> {
        test_parser!(source, array_expr_inner)
    }
}

use crate::alt;
use crate::brackets;
use crate::combinators::core::Combinator;
use crate::combinators::expr_list;
use crate::combinators::stmt::select_stmt;
use crate::context::ParserContext;
use crate::many;
use crate::paren;
use crate::seq;
use pg_ast::ExprNode;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use ExprNode::Array;
