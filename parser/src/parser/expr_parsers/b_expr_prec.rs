#[derive(Debug, Clone, Eq, PartialEq)]
enum Op {
    Typecast,
    Exponentiation,
    Multiplication,
    Division,
    Modulo,
    Addition,
    Subtraction,
    Less,
    Greater,
    Equals,
    GreaterEquals,
    LessEquals,
    NotEquals,
    IsExpr,
    QualifiedOperator(QualifiedOperator),
}

impl Op {
    fn precedence(&self) -> i16 {
        self.associativity().precedence()
    }

    fn associativity(&self) -> Associativity {
        use Op::*;

        match self {
            Typecast => Left(6),
            Exponentiation => Left(5),
            Multiplication | Division | Modulo => Left(4),
            Addition | Subtraction => Left(3),
            QualifiedOperator(_) => Left(2),
            Less | Greater | Equals | GreaterEquals | LessEquals | NotEquals => Non(1),
            IsExpr => Non(0),
        }
    }
}

impl Parser<'_> {
    pub(super) fn b_expr_prec(&mut self, prec: i16) -> ScanResult<ExprNode> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::b_expr_prec";

        // Precedence climbing

        let mut expr = self.b_expr_primary()?;

        let mut max_prec = 6;
        loop {
            let Some(op) = self.b_expr_op(prec, max_prec).optional()? else { break };
            let assoc = op.associativity();

            max_prec = assoc.max_precedence();

            if op == Op::Typecast {
                let type_name = self.type_name().required(fn_info!(FN_NAME))?;
                expr = TypecastExpr::new(type_name, expr).into();
                continue
            }

            if op == Op::IsExpr {
                expr = self.expr_is(expr)?;
                continue
            }

            let right = self.b_expr_prec(assoc.right_precedence()).required(fn_info!(FN_NAME))?;

            expr = match op {
                Op::Exponentiation => ExprNode::exponentiation(expr, right),
                Op::Multiplication => ExprNode::multiplication(expr, right),
                Op::Division => ExprNode::division(expr, right),
                Op::Modulo => ExprNode::modulo(expr, right),
                Op::Addition => ExprNode::addition(expr, right),
                Op::Subtraction => ExprNode::subtraction(expr, right),
                Op::Less => ExprNode::less(expr, right),
                Op::Greater => ExprNode::greater(expr, right),
                Op::Equals => ExprNode::equals(expr, right),
                Op::GreaterEquals => ExprNode::greater_equals(expr, right),
                Op::LessEquals => ExprNode::less_equals(expr, right),
                Op::NotEquals => ExprNode::not_equals(expr, right),
                _ => panic!("unexpected operator {op:?}")
            };
        }

        Ok(expr)
    }

    fn b_expr_op(&mut self, min_prec: i16, max_prec: i16) -> ScanResult<Op> {

        let range = min_prec..=max_prec;

        if range.contains(&2) {
            if let Some(op) = self.qual_op().no_match_to_option()? {
                return Ok(Op::QualifiedOperator(op))
            }
        }

        self.buffer.consume(|tok| {
            let op = match tok {
                Operator(Typecast) => Op::Typecast,
                Operator(Circumflex) => Op::Exponentiation,
                Operator(Mul) => Op::Multiplication,
                Operator(Div) => Op::Division,
                Operator(Percent) => Op::Modulo,
                Operator(Plus) => Op::Addition,
                Operator(Minus) => Op::Subtraction,
                Operator(Less) => Op::Less,
                Operator(Greater) => Op::Greater,
                Operator(Equals) => Op::Equals,
                Operator(GreaterEquals) => Op::GreaterEquals,
                Operator(LessEquals) => Op::LessEquals,
                Operator(NotEquals) => Op::NotEquals,
                Keyword(Is) => Op::IsExpr,
                _ => return None,
            };

            if range.contains(&op.associativity().precedence()) {
                Some(op)
            }
            else {
                None
            }
        })
    }

    fn expr_is(&mut self, left: ExprNode) -> ScanResult<ExprNode> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::expr_is";

        /*
            IS DISTINCT FROM b_expr
            IS NOT DISTINCT FROM b_expr
            IS DOCUMENT_P
            IS NOT DOCUMENT_P
        */

        let not_expr = self.buffer.consume_kw_eq(Not)
            .optional()?
            .is_some();

        let kw = self.buffer.consume_kw(|kw| matches!(kw, Document | Distinct))
            .required(fn_info!(FN_NAME))?;

        if kw == Document {
            let mut expr = ExprNode::is_xml_document(left);
            if not_expr {
                expr = ExprNode::not(expr)
            }
            return Ok(expr)
        }

        // Distinct
        self.buffer.consume_kw_eq(FromKw).required(fn_info!(FN_NAME))?;

        let assoc = Op::IsExpr.associativity();
        let right = self.b_expr_prec(assoc.right_precedence()).required(fn_info!(FN_NAME))?;

        let expr = if not_expr {
            ExprNode::not_distinct(left, right)
        }
        else {
            ExprNode::distinct(left, right)
        };

        Ok(expr)
    }

    fn b_expr_primary(&mut self) -> ScanResult<ExprNode> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::b_expr_primary";

        /*
            qual_Op b_expr(3)
            c_expr
            '-' b_expr(6)
            '+' b_expr(6)
        */

        if let Some(op) = self.qual_op().no_match_to_option()? {
            let prec = Left(2).right_precedence();
            let right = self.b_expr_prec(prec).required(fn_info!(FN_NAME))?;
            let expr = UnaryExpr::new(op, right);
            return Ok(expr.into())
        }

        // TODO: c_expr()

        let op = self.sign()?;

        let prec = Right(6).right_precedence();
        let right = self.b_expr_prec(prec).required(fn_info!(FN_NAME))?;

        let expr = if op == Plus {
            UnaryExpr::unary_plus(right)
        }
        else {
            UnaryExpr::negation(right)
        };

        Ok(expr.into())
    }
}

use crate::{
    lexer::{
        Keyword::{Distinct, Document, FromKw, Is, Not},
        OperatorKind::*,
        RawTokenKind::{Keyword, Operator}
    },
    parser::{
        ast_node::{ExprNode, QualifiedOperator, TypecastExpr, UnaryExpr},
        expr_parsers::associativity::Associativity::{self, Left, Non, Right},
        result::{
            Optional,
            Required,
            ScanResult,
            ScanResultTrait
        },
        token_buffer::TokenConsumer,
        Parser,
    }
};
use postgres_basics::fn_info;
