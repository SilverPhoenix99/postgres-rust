pub(in crate::combinators) fn a_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {
    a_expr_prec(0).parse(ctx)
}

fn a_expr_prec(prec: u8) -> impl Fn(&mut ParserContext) -> scan::Result<ExprNode> {
    move |ctx| {
        // TODO
        a_expr_primary(ctx)
    }
}

fn a_expr_primary(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
          ( '-' | '+' ) a_expr  // %right(11)
        | qual_Op a_expr        // %left(6)
        | NOT a_expr            // %right(2)
        | DEFAULT
        | TODO: UNIQUE opt_unique_null_treatment select_with_parens
        | c_expr
        | TODO: row OVERLAPS row
    */

    alt!(
        seq!(additive_op, a_expr_prec(11)).map(|(op, rhs)|
            UnaryExpr::new(op, rhs).into()
        ),
        seq!(qual_op, a_expr_prec(6)).map(|(op, rhs)|
            UnaryExpr::new(op, rhs).into()
        ),
        seq!(Not, a_expr_prec(2)).map(|(_, rhs)|
            BoolExpr::Not(rhs.into()).into()
        ),
        DefaultKw.map(|_| DefaultExpr),
        expr_primary
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::BooleanConst,
        ExprNode::IntegerConst,
        Operator::Addition,
        Operator::Subtraction,
    };
    use test_case::test_case;


    #[test_case("+ 1" => Ok(
        UnaryExpr::new(Addition, IntegerConst(1)).into()
    ))]
    #[test_case("operator(-) 2" => Ok(
        UnaryExpr::new(Subtraction, IntegerConst(2)).into()
    ))]
    #[test_case("not true" => Ok(
        BoolExpr::Not(BooleanConst(true).into()).into()
    ))]
    #[test_case("default" => Ok(DefaultExpr))]
    #[test_case("3" => Ok(IntegerConst(3)))]
    fn test_a_expr_primary(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, a_expr_primary)
    }
}

use crate::alt;
use crate::combinators::additive_op;
use crate::combinators::core::Combinator;
use crate::combinators::expr::expr_primary;
use crate::combinators::qual_op;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::BoolExpr;
use pg_ast::ExprNode;
use pg_ast::ExprNode::DefaultExpr;
use pg_ast::UnaryExpr;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::Not;
use pg_parser_core::scan;
