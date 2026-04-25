pub(in crate::combinators) fn b_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {
    b_expr_prec(0).parse(ctx)
}

fn b_expr_prec(prec: u8) -> impl Fn(&mut ParserContext) -> scan::Result<ExprNode> {
    move |ctx| {

        /*
            Converted to precedence climbing.

            b_expr:
                  b_expr_primary
                | b_expr TYPECAST Typename            // %left(7)
                | b_expr '^' b_expr                   // %left(5)
                | b_expr '*' b_expr                   // %left(4)
                | b_expr '/' b_expr                   // %left(4)
                | b_expr '%' b_expr                   // %left(4)
                | b_expr '-' b_expr                   // %left(3)
                | b_expr '+' b_expr                   // %left(3)
                | b_expr qual_Op b_expr               // %left(2)
                | b_expr RIGHT_ARROW b_expr           // %left(2)
                | b_expr '|' b_expr                   // %left(2)
                | b_expr '<' b_expr                   // %nonassoc(1)
                | b_expr '=' b_expr                   // %nonassoc(1)
                | b_expr '>' b_expr                   // %nonassoc(1)
                | b_expr GREATER_EQUALS b_expr        // %nonassoc(1)
                | b_expr LESS_EQUALS b_expr           // %nonassoc(1)
                | b_expr NOT_EQUALS b_expr            // %nonassoc(1)
                | b_expr IS DISTINCT FROM b_expr      // %nonassoc(0)
                | b_expr IS DOCUMENT                  // %nonassoc(0)
                | b_expr IS NOT DISTINCT FROM b_expr  // %nonassoc(0)
                | b_expr IS NOT DOCUMENT              // %nonassoc(0)
        */

        let mut lhs = b_expr_primary(ctx)?;

        loop {

            if prec <= 7 && let Some((_, rhs)) = {
                seq!(Typecast, typename)
                    .parse(ctx)
                    .optional()?
            } {
                lhs = TypecastExpr::new(lhs, rhs).into();
                continue
            }

            if prec <= 5 && let Some((_, rhs)) = {
                seq!(Circumflex, b_expr_prec(6))
                    .parse(ctx)
                    .optional()?
            } {
                lhs = BinaryExpr::new(Exponentiation, lhs, rhs).into();
                continue
            }

            if prec <= 4 && let Some((op, rhs)) = {
                seq!(multiplicative_op, b_expr_prec(5))
                    .parse(ctx)
                    .optional()?
            } {
                lhs = BinaryExpr::new(op, lhs, rhs).into();
                continue
            }

            if prec <= 3 && let Some((op, rhs)) = {
                seq!(additive_op, b_expr_prec(4))
                    .parse(ctx)
                    .optional()?
            } {
                lhs = BinaryExpr::new(op, lhs, rhs).into();
                continue
            }

            if prec <= 2 && let Some((op, rhs)) = {
                seq!(
                    alt!(
                        Op::RightArrow.map(|_| RightArrow.into()),
                        Op::Pipe.map(|_| Pipe.into()),
                        qual_op
                    ),
                    b_expr_prec(3)
                )
                    .parse(ctx)
                    .optional()?
            } {
                lhs = BinaryExpr::new(op, lhs, rhs).into();
                continue
            }

            if prec <= 1 && let Some((op, rhs)) = {
                seq!(boolean_op, b_expr_prec(2))
                    .parse(ctx)
                    .optional()?
            } {
                return Ok(BinaryExpr::new(op, lhs, rhs).into())
            }

            if prec == 0 && let Some((_, not, rhs)) = {
                seq!(
                    Is,
                    Not.optional(),
                    alt!(
                        Document.map(|_| None),
                        seq!(Distinct, FromKw, b_expr_prec(1)).map(|(_, _, rhs)| Some(rhs))
                    )
                )
                    .parse(ctx)
                    .optional()?
            } {

                let expr = match (rhs, not.is_some()) {
                    (Some(rhs), false) => IsDistinct((lhs, rhs).into()),
                    (Some(rhs), true) => IsNotDistinct((lhs, rhs).into()),
                    (None, false) => IsDocument(lhs.into()),
                    (None, true) => IsNotDocument(lhs.into()),
                };

                return Ok(expr)
            }

            // No more matches
            return Ok(lhs)
        }
    }
}

fn b_expr_primary(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
          '-' b_expr     // %right(7) == %prec UMINUS
        | '+' b_expr     // %right(7) == %prec UMINUS
        | qual_Op b_expr // %left(3) == %prec Op
        | c_expr
    */

    alt!(
        seq!(Minus, b_expr_prec(7))
            .map(|(_, rhs)| match rhs {
                IntegerConst(int) => IntegerConst(-int),
                NumericConst { value, radix, negative } => NumericConst {
                    value,
                    radix,
                    negative: !negative,
                },
                rhs => UnaryExpr::new(Subtraction, rhs).into()
            }),
        seq!(Plus, b_expr_prec(7))
            .map(|(_, rhs)| UnaryExpr::new(Addition, rhs).into()),
        seq!(qual_op, b_expr_prec(3))
            .map(|(op, rhs)| UnaryExpr::new(op, rhs).into()),
        expr_primary
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst as Int;
    use pg_ast::Operator::Equals;
    use pg_ast::Operator::Modulo;
    use pg_ast::Operator::Multiplication;
    use pg_ast::TypeName::Int4;
    use pg_ast::TypeName::Varchar;
    use test_case::test_matrix;

    #[test_matrix("- 1" => Ok(Int(-1)); "unary minus")]
    #[test_matrix("+ 2" => Ok(UnaryExpr::new(Addition, Int(2)).into()); "unary plus")]
    #[test_matrix("operator(+) 3" => Ok(UnaryExpr::new(Addition, Int(3)).into()); "unary qual_op")]
    #[test_matrix("4" => Ok(Int(4)); "expr primary")]
    fn test_b_expr_primary(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, b_expr_primary)
    }

    /*
        Single expressions
     */
    #[test_matrix("1" => Ok(Int(1)))]
    #[test_matrix("1::varchar" => Ok(TypecastExpr::new(Int(1), Varchar { max_length: None }).into()))]
    #[test_matrix("1 ^ 2"  => Ok(BinaryExpr::new(Exponentiation, Int(1), Int(2)).into()))]
    #[test_matrix("3 % 4"  => Ok(BinaryExpr::new(Modulo,         Int(3), Int(4)).into()))]
    #[test_matrix("5 + 6"  => Ok(BinaryExpr::new(Addition,       Int(5), Int(6)).into()))]
    #[test_matrix("7 -> 8" => Ok(BinaryExpr::new(RightArrow,     Int(7), Int(8)).into()))]
    #[test_matrix("9 operator(-) 10" => Ok(BinaryExpr::new(Subtraction, Int(9), Int(10)).into()))]
    #[test_matrix("1 is document" => Ok(IsDocument(Int(1).into())))]
    #[test_matrix("2 is not document" => Ok(IsNotDocument(Int(2).into())))]
    #[test_matrix("3 is distinct from 4" => Ok(IsDistinct((Int(3), Int(4)).into())))]
    #[test_matrix("5 is not distinct from 6" => Ok(IsNotDistinct((Int(5), Int(6)).into())))]
    /*
        Multiple expressions
    */
    #[test_matrix("- 2 ^ 4" => Ok(
        BinaryExpr::new(Exponentiation, Int(-2), Int(4))
            .into()
    ))]
    #[test_matrix("operator(-) 5 ^ 3" => Ok(
        UnaryExpr::new(Subtraction,
            BinaryExpr::new(Exponentiation, Int(5), Int(3))
        ).into()
    ))]
    #[test_matrix("1 + 2 * 3" => Ok(
        BinaryExpr::new(Addition,
            Int(1),
            BinaryExpr::new(Multiplication, Int(2), Int(3))
        ).into()
    ))]
    #[test_matrix("4 - 5 - 6" => Ok(
        BinaryExpr::new(Subtraction,
            BinaryExpr::new(Subtraction, Int(4), Int(5)),
            Int(6)
        ).into()
    ))]
    #[test_matrix("7 ^ 8 ^ 9" => Ok(
        BinaryExpr::new(Exponentiation,
            BinaryExpr::new(Exponentiation, Int(7), Int(8)),
            Int(9)
        ).into()
    ))]
    // Won't match "= 3"
    #[test_matrix("1 = 2 = 3" => Ok(
        BinaryExpr::new(Equals, Int(1), Int(2)).into()
    ))]
    #[test_matrix("1::varchar::int" => Ok(
        TypecastExpr::new(
            TypecastExpr::new(Int(1), Varchar { max_length: None }).into(),
            Int4
        ).into()
    ))]
    #[test_matrix("1::varchar ^ 2" => Ok(
        BinaryExpr::new(Exponentiation,
            TypecastExpr::new(Int(1), Varchar { max_length: None }),
            Int(2)
        ).into()
    ))]
    #[test_matrix("1 -> 2 + 3" => Ok(
        BinaryExpr::new(RightArrow,
            Int(1),
            BinaryExpr::new(Addition, Int(2), Int(3))
        ).into()
    ))]
    fn test_b_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, b_expr)
    }
}

use super::expr_primary;
use crate::alt;
use crate::combinators::additive_op;
use crate::combinators::boolean_op;
use crate::combinators::core::Combinator;
use crate::combinators::multiplicative_op;
use crate::combinators::qual_op;
use crate::combinators::typename;
use crate::seq;
use crate::ParserContext;
use pg_ast::BinaryExpr;
use pg_ast::ExprNode;
use pg_ast::ExprNode::IntegerConst;
use pg_ast::ExprNode::IsDistinct;
use pg_ast::ExprNode::IsDocument;
use pg_ast::ExprNode::IsNotDistinct;
use pg_ast::ExprNode::IsNotDocument;
use pg_ast::ExprNode::NumericConst;
use pg_ast::Operator::Addition;
use pg_ast::Operator::Exponentiation;
use pg_ast::Operator::Pipe;
use pg_ast::Operator::RightArrow;
use pg_ast::Operator::Subtraction;
use pg_ast::TypecastExpr;
use pg_ast::UnaryExpr;
use pg_lexer::Keyword::Distinct;
use pg_lexer::Keyword::Document;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::Is;
use pg_lexer::Keyword::Not;
use pg_lexer::OperatorKind as Op;
use pg_lexer::OperatorKind::Circumflex;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::Plus;
use pg_lexer::OperatorKind::Typecast;
use pg_parser_core::scan;
use pg_parser_core::Optional;
