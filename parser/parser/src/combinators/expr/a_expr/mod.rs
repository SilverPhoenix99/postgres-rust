pg_basics::reexport! {
    a_expr_primary
}

pub(in crate::combinators) fn a_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {
    a_expr_prec(0).parse(ctx)
}

fn a_expr_prec(prec: u8) -> impl Fn(&mut ParserContext) -> scan::Result<ExprNode> {
    move |ctx| {

        /*
            Converted to precedence climbing.

            a_expr:
                  ✅ a_expr_primary
                | ✅ a_expr TYPECAST Typename                                              // %left(14)
                | a_expr AT ( LOCAL | TIME ZONE a_expr )                                // %left(12)
                | a_expr COLLATE any_name                                               // %left(10)

                | a_expr '^' a_expr                                                     // %left(9)
                | a_expr '^' sub_type '(' SelectStmt | a_expr ')'                       // %left(6)

                | a_expr additive_op a_expr                                             // %left(8)
                | a_expr additive_op sub_type '(' SelectStmt | a_expr ')'               // %left(6)

                | a_expr multiplicative_op a_expr                                       // %left(7)
                | a_expr multiplicative_op sub_type '(' SelectStmt | a_expr ')'         // %left(6)

                | a_expr boolean_op sub_type '(' SelectStmt | a_expr ')'                // %left(6)
                | a_expr boolean_op a_expr                                              // %nonassoc(4)

                | a_expr ILIKE sub_type '(' SelectStmt | a_expr ')'                     // %left(6)
                | a_expr ILIKE a_expr ( ESCAPE a_expr )?                                // %nonassoc(5)

                | a_expr IN '(' expr_list ')'                                           // %left(13)
                | a_expr IN '(' SelectStmt ')'                                          // %nonassoc(5)

                | a_expr LIKE sub_type '(' SelectStmt | a_expr ')'                      // %left(6)
                | a_expr LIKE a_expr ( ESCAPE a_expr )?                                 // %nonassoc(5)

                | a_expr NOT IN '(' expr_list ')'                                       // %left(13)
                | a_expr NOT IN '(' SelectStmt ')'                                      // %nonassoc(5)

                | a_expr NOT BETWEEN ( ASYMMETRIC | SYMMETRIC )? b_expr AND a_expr      // %nonassoc(5)
                | a_expr NOT ILIKE sub_type '(' SelectStmt | a_expr ')'                 // %left(6)
                | a_expr NOT ILIKE a_expr ( ESCAPE a_expr )?                            // %nonassoc(5)
                | a_expr NOT LIKE sub_type '(' SelectStmt | a_expr ')'                  // %left(6)
                | a_expr NOT LIKE a_expr ( ESCAPE a_expr )?                             // %nonassoc(5)
                | a_expr NOT SIMILAR TO a_expr ( ESCAPE a_expr )?                       // %nonassoc(5)

                | a_expr misc_op sub_type '(' SelectStmt | a_expr ')'                   // %left(6)
                | a_expr misc_op a_expr                                                 // %left(6)

                | a_expr BETWEEN ( ASYMMETRIC | SYMMETRIC )? b_expr AND a_expr          // %nonassoc(5)
                | a_expr SIMILAR TO a_expr ( ESCAPE a_expr )?                           // %nonassoc(5)
                | a_expr ISNULL                                                         // %nonassoc(3)
                | a_expr NOTNULL                                                        // %nonassoc(3)

                | a_expr IS DISTINCT FROM a_expr                                        // %nonassoc(3)
                | a_expr IS DOCUMENT                                                    // %nonassoc(3)
                | a_expr IS FALSE                                                       // %nonassoc(3)
                | a_expr IS JSON json_expr                                              // %nonassoc(3)
                | a_expr IS NORMALIZED                                                  // %nonassoc(3)
                | a_expr IS NOT DISTINCT FROM a_expr                                    // %nonassoc(3)
                | a_expr IS NOT DOCUMENT                                                // %nonassoc(3)
                | a_expr IS NOT FALSE                                                   // %nonassoc(3)
                | a_expr IS NOT JSON json_expr                                          // %nonassoc(3)
                | a_expr IS NOT NORMALIZED                                              // %nonassoc(3)
                | a_expr IS NOT NULL                                                    // %nonassoc(3)
                | a_expr IS NOT TRUE                                                    // %nonassoc(3)
                | a_expr IS NOT unicode_normal_form NORMALIZED                          // %nonassoc(3)
                | a_expr IS NOT UNKNOWN                                                 // %nonassoc(3)
                | a_expr IS NULL                                                        // %nonassoc(3)
                | a_expr IS TRUE                                                        // %nonassoc(3)
                | a_expr IS unicode_normal_form NORMALIZED                              // %nonassoc(3)
                | a_expr IS UNKNOWN                                                     // %nonassoc(3)

                | ✅ a_expr AND a_expr                                                     // %left(1)
                | ✅ a_expr OR a_expr                                                      // %left(0)
        */

        let mut lhs = a_expr_primary(ctx)?;

        loop {

            if prec <= 14 && let Some((_, rhs)) = seq!(Typecast, typename).parse(ctx).optional()? {
                lhs = TypecastExpr::new(lhs, rhs).into();
                continue
            }

            // TODO

            if prec <= 1 && let Some((_, rhs)) = seq!(And, a_expr_prec(2)).parse(ctx).optional()? {

                if let ExprNode::BoolExpr(BoolExpr::And(args)) = &mut lhs {
                    // Flatten "a AND b AND c ..." to a single BoolExpr on sight
                    args.push(rhs);
                }
                else {
                    lhs = BoolExpr::And(vec![lhs, rhs]).into();
                }

                continue
            }

            if prec == 0 && let Some((_, rhs)) = seq!(Or, a_expr_prec(1)).parse(ctx).optional()? {

                if let ExprNode::BoolExpr(BoolExpr::Or(args)) = &mut lhs {
                    // Flatten "a OR b OR c ..." to a single BoolExpr on sight
                    args.push(rhs);
                }
                else {
                    lhs = BoolExpr::Or(vec![lhs, rhs]).into();
                }

                continue
            }

            // No more matches
            return Ok(lhs)
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst as Int;
    use pg_ast::TypeName::Varchar;
    use test_case::test_matrix;

    /*
        Single expressions
     */
    #[test_matrix("1" => matches Ok(Int(1)))]
    #[test_matrix("1::varchar" => Ok(TypecastExpr::new(Int(1), Varchar { max_length: None }).into()))]
    /*
        Multiple expressions
    */
    #[test_matrix("1 and 2 and 3 or 4 or 5" => Ok(
        // ((1 AND 2 AND 3) OR 4 OR 5)
        BoolExpr::Or(vec![
            BoolExpr::And(vec![Int(1), Int(2), Int(3)]).into(),
            Int(4),
            Int(5),
        ]).into()
    ))]
    fn test_a_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, a_expr)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::typename;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::BoolExpr;
use pg_ast::ExprNode;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword::And;
use pg_lexer::Keyword::Or;
use pg_lexer::OperatorKind::Typecast;
use pg_parser_core::scan;
use pg_parser_core::Optional;
