mod associativity;

pg_basics::reexport! { pub(super)
    expr_primary,
    func_expr_common_subexpr,
}

pg_basics::reexport! {
    expr_const,
    indirection,
    unicode_normal_form,
}

pub(super) fn a_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {
    // TODO
    expr_primary(ctx)
}

pub(super) fn b_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {
    // TODO
    expr_primary(ctx)
}

fn b_expr_primary(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
          '-' b_expr
        | '+' b_expr
        | qual_Op b_expr
        | c_expr
    */

    alt!(
        seq!(Minus, b_expr /* %right(12) */)
            .map(|(_, expr)|
                UnaryExpr::new(Subtraction, expr).into()
            ),
        seq!(Plus, b_expr /* %right(12) */)
            .map(|(_, expr)|
                UnaryExpr::new(Addition, expr).into()
            ),
        seq!(qual_op, b_expr /* %left(12) */)
            .map(|(op, expr)|
                UnaryExpr::new(op, expr).into()
            ),
        expr_primary
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("- 1" => Ok(UnaryExpr::new(Subtraction, IntegerConst(1)).into()); "unary minus")]
    #[test_case("+ 2" => Ok(UnaryExpr::new(Addition, IntegerConst(2)).into()); "unary plus")]
    #[test_case("operator(+) 3" => Ok(UnaryExpr::new(Addition, IntegerConst(3)).into()); "unary qual_op")]
    #[test_case("4" => Ok(IntegerConst(4)); "expr primary")]
    fn test_b_expr_primary(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, b_expr_primary)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::operators::qual_op;
use crate::seq;
use crate::ParserContext;
use pg_ast::ExprNode;
use pg_ast::Operator::Addition;
use pg_ast::Operator::Subtraction;
use pg_ast::UnaryExpr;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::Plus;
use pg_parser_core::scan;
