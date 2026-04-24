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
        | UNIQUE opt_unique_null_treatment select_with_parens
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
        unique_predicate,
        expr_primary
    ).parse(ctx)
}

fn unique_predicate(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        UNIQUE opt_unique_null_treatment select_with_parens
    */

    let (Located(_, loc), ..) = seq!(
        located!(Unique),
        unique_null_treatment.optional(),
        paren!(select_stmt)
    ).parse(ctx)?;

    Err(UniquePredicateNotImplemented.at_location(loc).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::BooleanConst,
        pg_ast::ExprNode::IntegerConst,
        pg_ast::Operator::Addition,
        pg_ast::Operator::Subtraction,
        pg_elog::Error::Parser,
        pg_parser_core::scan::Error::ScanErr,
    };

    #[test_matrix("+ 1" => Ok(
        UnaryExpr::new(Addition, IntegerConst(1)).into()
    ))]
    #[test_matrix("operator(-) 2" => Ok(
        UnaryExpr::new(Subtraction, IntegerConst(2)).into()
    ))]
    #[test_matrix("not true" => Ok(
        BoolExpr::Not(BooleanConst(true).into()).into()
    ))]
    #[test_matrix("default" => Ok(DefaultExpr))]
    #[test_matrix("3" => Ok(IntegerConst(3)))]
    fn test_a_expr_primary(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, a_expr_primary)
    }

    #[test_matrix(
        [
            "unique (select 1)",
            "unique nulls distinct (select 2)",
            "unique nulls not distinct (select 3)",
        ]
        => ignore["select_stmt not implemented yet"] matches Err(ScanErr(
            Located(Parser(UniquePredicateNotImplemented), _)
        ))
    )]
    fn test_unique_predicate(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, unique_predicate)
    }
}

use crate::alt;
use crate::combinators::additive_op;
use crate::combinators::core::Combinator;
use crate::combinators::expr::expr_primary;
use crate::combinators::qual_op;
use crate::combinators::stmt::select_stmt;
use crate::combinators::unique_null_treatment;
use crate::context::ParserContext;
use crate::located;
use crate::paren;
use crate::seq;
use pg_ast::BoolExpr;
use pg_ast::ExprNode;
use pg_ast::ExprNode::DefaultExpr;
use pg_ast::UnaryExpr;
use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_elog::parser::Error::UniquePredicateNotImplemented;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::Not;
use pg_lexer::Keyword::Unique;
use pg_parser_core::scan;
