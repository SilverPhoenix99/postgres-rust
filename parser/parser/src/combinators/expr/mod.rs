pg_basics::reexport! { pub(super)
    expr_primary,
    func_expr_common_subexpr,
}

pg_basics::reexport! {
    expr_const,
    indirection,
    row,
    unicode_normal_form,
}

pub(super) fn a_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {
    // TODO
    expr_primary(ctx)
}

pub(super) fn b_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {
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
                | b_expr qual_Op b_expr               // %left(2) == %prec Op
                | b_expr RIGHT_ARROW b_expr           // %left(2)
                | b_expr '|' b_expr                   // %left(2)
                | b_expr '<' b_expr                   // %nonassoc(1)
                | b_expr '=' b_expr                   // %nonassoc(1)
                | b_expr '>' b_expr                   // %nonassoc(1)
                | b_expr GREATER_EQUALS b_expr        // %nonassoc(1)
                | b_expr LESS_EQUALS b_expr           // %nonassoc(1)
                | b_expr NOT_EQUALS b_expr            // %nonassoc(1)
                | b_expr IS DISTINCT FROM b_expr      // %nonassoc(0) == %prec IS
                | b_expr IS DOCUMENT                  // %nonassoc(0) == %prec IS
                | b_expr IS NOT DISTINCT FROM b_expr  // %nonassoc(0) == %prec IS
                | b_expr IS NOT DOCUMENT              // %nonassoc(0) == %prec IS
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
                seq!(
                    alt!(
                        Mul.map(|_| Multiplication),
                        Div.map(|_| Division),
                        Percent.map(|_| Modulo),
                    ),
                    b_expr_prec(5)
                )
                    .parse(ctx)
                    .optional()?
            } {
                lhs = BinaryExpr::new(op, lhs, rhs).into();
                continue
            }

            if prec <= 3 && let Some((op, rhs)) = {
                seq!(
                    alt!(
                        Minus.map(|_| Subtraction),
                        Plus.map(|_| Addition),
                    ),
                    b_expr_prec(4)
                )
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
                seq!(
                    alt!(
                        Op::Equals.map(|_| Equals),
                        Op::Greater.map(|_| Greater),
                        Op::GreaterEquals.map(|_| GreaterEquals),
                        Op::Less.map(|_| Less),
                        Op::LessEquals.map(|_| LessEquals),
                        Op::NotEquals.map(|_| NotEquals),
                    ),
                    b_expr_prec(2)
                )
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
    use pg_ast::ExprNode;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::IntegerConst as Int,
        TypeName::Varchar,
    };
    use pg_parser_core::scan;
    use test_case::test_case;

    #[test_case("- 1" => Ok(Int(-1)); "unary minus")]
    #[test_case("+ 2" => Ok(UnaryExpr::new(Addition, Int(2)).into()); "unary plus")]
    #[test_case("operator(+) 3" => Ok(UnaryExpr::new(Addition, Int(3)).into()); "unary qual_op")]
    #[test_case("4" => Ok(Int(4)); "expr primary")]
    fn test_b_expr_primary(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, b_expr_primary)
    }

    #[test_case("1::varchar" => Ok(TypecastExpr::new(Int(1), Varchar { max_length: None }).into()))]
    #[test_case("1 ^ 2"  => Ok(BinaryExpr::new(Exponentiation, Int(1), Int(2)).into()))]
    #[test_case("3 % 4"  => Ok(BinaryExpr::new(Modulo,         Int(3), Int(4)).into()))]
    #[test_case("5 + 6"  => Ok(BinaryExpr::new(Addition,       Int(5), Int(6)).into()))]
    #[test_case("7 -> 8" => Ok(BinaryExpr::new(RightArrow,     Int(7), Int(8)).into()))]
    #[test_case("9 operator(-) 10" => Ok(BinaryExpr::new(Subtraction, Int(9), Int(10)).into()))]
    #[test_case("1 is document" => Ok(IsDocument(Int(1).into())))]
    #[test_case("2 is not document" => Ok(IsNotDocument(Int(2).into())))]
    #[test_case("3 is distinct from 4" => Ok(IsDistinct((Int(3), Int(4)).into())))]
    #[test_case("5 is not distinct from 6" => Ok(IsNotDistinct((Int(5), Int(6)).into())))]
    fn test_b_expr_single_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, b_expr)
    }

    #[test_case("- 2 ^ 4" => Ok(
        BinaryExpr::new(Exponentiation, Int(-2), Int(4))
            .into()
    ))]
    #[test_case("operator(-) 5 ^ 3" => Ok(
        UnaryExpr::new(Subtraction,
            BinaryExpr::new(Exponentiation, Int(5), Int(3))
        ).into()
    ))]
    fn test_b_expr_multiple_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, b_expr)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
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
use pg_ast::Operator::Division;
use pg_ast::Operator::Equals;
use pg_ast::Operator::Exponentiation;
use pg_ast::Operator::Greater;
use pg_ast::Operator::GreaterEquals;
use pg_ast::Operator::Less;
use pg_ast::Operator::LessEquals;
use pg_ast::Operator::Modulo;
use pg_ast::Operator::Multiplication;
use pg_ast::Operator::NotEquals;
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
use pg_lexer::OperatorKind::Div;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::Mul;
use pg_lexer::OperatorKind::Percent;
use pg_lexer::OperatorKind::Plus;
use pg_lexer::OperatorKind::Typecast;
use pg_parser_core::scan;
use pg_parser_core::Optional;
