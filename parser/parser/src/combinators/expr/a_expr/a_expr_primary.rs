pub(super) fn a_expr_primary(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
          ( '-' | '+' ) a_expr  // %right(11)
        | qual_Op a_expr        // %left(6)
        | NOT a_expr            // %right(2)
        | DEFAULT
        | UNIQUE opt_unique_null_treatment select_with_parens
        | c_expr
        | row OVERLAPS row
    */

    let Located(expr, loc) = located!(alt!(
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
    )).parse(ctx)?;

    let Row(lhs) = expr else {
        return Ok(expr)
    };

    row_overlaps_expr(ctx, lhs, loc)
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

fn row_overlaps_expr(ctx: &mut ParserContext, lhs: Option<Vec<ExprNode>>, lhs_loc: Location) -> scan::Result<ExprNode> {

    /*
        row OVERLAPS row
    */

    let Some(Located(rhs, rhs_loc)) = overlaps_row(ctx).optional()? else {
        return Ok(Row(lhs))
    };

    let mut lhs = lhs.unwrap_or_default();
    let mut rhs = rhs.unwrap_or_default();

    if let ([l1, l2], [r1, r2]) = (lhs.as_mut_slice(), rhs.as_mut_slice()) {
        let lhs = (mem::replace(l1, DefaultExpr), mem::replace(l2, DefaultExpr));
        let rhs = (mem::replace(r1, DefaultExpr), mem::replace(r2, DefaultExpr));
        let expr = RowOverlaps::new(lhs, rhs).into();
        return Ok(expr)
    }

    let err = if lhs.len() != 2 {
        WrongNumberOfLeftOverlapsParameters.at_location(lhs_loc)
    }
    else {
        WrongNumberOfRightOverlapsParameters.at_location(rhs_loc)
    };

    Err(err.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::BooleanConst;
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::Operator::Addition;
    use pg_ast::Operator::Subtraction;
    use pg_elog::Error::Parser;
    use pg_parser_core::scan::Error::ScanErr;
    use test_case::test_matrix;

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
    #[test_matrix(
        [
            "row(1, 2) overlaps row(3, 4)",
            "row(1, 2) overlaps (3, 4)",
            "(1, 2) overlaps row(3, 4)",
            "(1, 2) overlaps (3, 4)",
        ]
        => Ok(RowOverlaps::new(
            (IntegerConst(1), IntegerConst(2)),
            (IntegerConst(3), IntegerConst(4)),
        ).into())
    )]
    #[test_matrix(
        [
            "row() overlaps row()",
            "row() overlaps row(1)",
            "row() overlaps (1, 2)",
            "row() overlaps (1, 2, 3)",
            "row(1) overlaps row()",
            "row(1) overlaps row(2)",
            "row(1) overlaps (2, 3)",
            "row(1) overlaps (2, 3, 4)",
            "(1, 2, 3) overlaps row()",
            "(1, 2, 3) overlaps row(4)",
            "(1, 2, 3) overlaps (4, 5)",
            "(1, 2, 3) overlaps (4, 5, 6)",
        ]
        => matches Err(ScanErr(
            Located(Parser(WrongNumberOfLeftOverlapsParameters), _)
        ))
    )]
    #[test_matrix(
        [
            "(1, 2) overlaps row()",
            "(1, 2) overlaps row(3)",
            "(1, 2) overlaps (3, 4, 5)",
        ]
        => matches Err(ScanErr(
            Located(Parser(WrongNumberOfRightOverlapsParameters), _)
        ))
    )]
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

use super::a_expr_prec;
use crate::alt;
use crate::combinators::additive_op;
use crate::combinators::core::Combinator;
use crate::combinators::expr::expr_primary;
use crate::combinators::expr::overlaps_row;
use crate::combinators::qual_op;
use crate::combinators::stmt::select_stmt;
use crate::combinators::unique_null_treatment;
use crate::context::ParserContext;
use crate::located;
use crate::paren;
use crate::seq;
use core::mem;
use pg_ast::BoolExpr;
use pg_ast::ExprNode;
use pg_ast::ExprNode::DefaultExpr;
use pg_ast::ExprNode::Row;
use pg_ast::RowOverlaps;
use pg_ast::UnaryExpr;
use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_basics::Location;
use pg_elog::parser::Error::UniquePredicateNotImplemented;
use pg_elog::parser::Error::WrongNumberOfLeftOverlapsParameters;
use pg_elog::parser::Error::WrongNumberOfRightOverlapsParameters;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::Not;
use pg_lexer::Keyword::Unique;
use pg_parser_core::scan;
use pg_parser_core::Optional;
