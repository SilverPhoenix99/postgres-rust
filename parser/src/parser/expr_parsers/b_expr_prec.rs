impl Parser<'_> {
    pub(super) fn b_expr_prec(&mut self, prec: i16) -> ScanResult<ExprNode> {

        // Precedence climbing

        let mut expr = self.b_expr_primary()?;

        let mut max_prec = 6;
        loop {
            let range = prec..=max_prec;
            let op = self.buffer.consume(|tok| match tok {
                Typecast if range.contains(&6) => Some((Typecast, Associativity::Left(6))),
                Circumflex if range.contains(&5) => Some((Circumflex, Associativity::Left(5))),
                op @ (Mul | Div | Percent) if range.contains(&4) => Some((*op, Associativity::Left(4))),
                op @ (Plus | Minus) if range.contains(&3) => Some((*op, Associativity::Left(3))),
                // TODO: ( ?(p<=2) qual_Op b_expr(3) )*
                op @ (Less | Greater | Equals | GreaterEquals | LessEquals | NotEquals) if range.contains(&1) => {
                    Some((*op, Associativity::Non(1)))
                },
                Keyword(Is) if range.contains(&0) => Some((Keyword(Is), Associativity::Non(0))),
                _ => None,
            });

            let Some((op, assoc)) = op.optional()? else { break };

            max_prec = assoc.max_precedence();

            if op == Typecast {
                // TODO: let typename = self.typename().required()?; // -> TypeName
                expr = ExprNode::Typecast((/* tree, typename */));
                continue
            }

            if matches!(op, Keyword(_)) { // `Is`
                let not_expr = self.buffer.consume_kw_eq(Not)
                    .optional()?
                    .is_some();

                let kw = self.buffer.consume_kws(|kw| matches!(kw, Document | Distinct))
                    .required()?;

                expr = if kw == Document {
                    let mut right = ExprNode::is_xml_document(expr);
                    if not_expr {
                        right = ExprNode::not(right)
                    }
                    right
                }
                else {
                    // Distinct
                    self.buffer.consume_kw_eq(From).required()?;
                    let right = self.b_expr_prec(assoc.right_prec()).required()?;

                    if not_expr {
                        ExprNode::not_distinct(expr, right)
                    }
                    else {
                        ExprNode::distinct(expr, right)
                    }
                };

                continue
            }

            let right = self.b_expr_prec(assoc.right_prec()).required()?;

            expr = match op {
                Circumflex => ExprNode::exponentiation(expr, right),
                Mul => ExprNode::multiplication(expr, right),
                Div => ExprNode::division(expr, right),
                Percent => ExprNode::modulo(expr, right),
                Plus => ExprNode::addition(expr, right),
                Minus => ExprNode::subtraction(expr, right),
                Less => ExprNode::less(expr, right),
                Greater => ExprNode::greater(expr, right),
                Equals => ExprNode::equals(expr, right),
                GreaterEquals => ExprNode::greater_equals(expr, right),
                LessEquals => ExprNode::less_equals(expr, right),
                NotEquals => ExprNode::not_equals(expr, right),
                _ => panic!("unexpected operator {op:?}")
            };
        }

        Ok(expr)
    }

    fn b_expr_primary(&self) -> ScanResult<ExprNode> {
        todo!()
    }
}

use crate::lexer::Keyword::{Distinct, Document, From, Is, Not};
use crate::lexer::TokenKind::*;
use crate::parser::ast_node::ExprNode;
use crate::parser::expr_parsers::associativity::Associativity;
use crate::parser::result::{ScanResult, ScanResultTrait};
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::Parser;
