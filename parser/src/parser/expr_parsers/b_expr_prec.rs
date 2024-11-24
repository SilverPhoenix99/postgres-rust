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

        // Precedence climbing

        let mut expr = self.b_expr_primary()?;

        let mut max_prec = 6;
        loop {
            let Some(op) = self.b_expr_op(prec, max_prec).optional()? else { break };
            let assoc = op.associativity();

            max_prec = assoc.max_precedence();

            if op == Op::Typecast {
                let type_name = self.type_name().required()?;
                expr = TypecastExpr::new(type_name, expr).into();
                continue
            }

            if op == Op::IsExpr {
                expr = self.expr_is(expr)?;
                continue
            }

            let right = self.b_expr_prec(assoc.right_precedence()).required()?;

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
            if let Some(op) = qual_op().maybe_match().parse(&mut self.buffer)? {
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

        /*
            IS DISTINCT FROM b_expr
            IS NOT DISTINCT FROM b_expr
            IS DOCUMENT_P
            IS NOT DOCUMENT_P
        */

        let not_expr = Not.optional()
            .parse(&mut self.buffer)?
            .is_some();

        let kw = keyword_if(|kw| matches!(kw, Document | Distinct))
            .required()
            .parse(&mut self.buffer)?;

        if kw == Document {
            let mut expr = ExprNode::is_xml_document(left);
            if not_expr {
                expr = ExprNode::not(expr)
            }
            return Ok(expr)
        }

        // Distinct
        FromKw.required()
            .parse(&mut self.buffer)?;

        let assoc = Op::IsExpr.associativity();
        let right = self.b_expr_prec(assoc.right_precedence()).required()?;

        let expr = if not_expr {
            ExprNode::not_distinct(left, right)
        }
        else {
            ExprNode::distinct(left, right)
        };

        Ok(expr)
    }

    fn b_expr_primary(&mut self) -> ScanResult<ExprNode> {

        /*
            qual_Op b_expr(3)
            c_expr
            '-' b_expr(6)
            '+' b_expr(6)
        */

        if let Some(op) = qual_op().maybe_match().parse(&mut self.buffer)? {
            let prec = Left(2).right_precedence();
            let right = self.b_expr_prec(prec).required()?;
            let expr = UnaryExpr::new(op, right);
            return Ok(expr.into())
        }

        // TODO: c_expr()

        let op = sign().parse(&mut self.buffer)?;

        let prec = Right(6).right_precedence();
        let right = self.b_expr_prec(prec).required()?;

        let expr = if op == Plus {
            UnaryExpr::unary_plus(right)
        }
        else {
            UnaryExpr::negation(right)
        };

        Ok(expr.into())
    }
}

use crate::lexer::Keyword::Distinct;
use crate::lexer::Keyword::Document;
use crate::lexer::Keyword::FromKw;
use crate::lexer::Keyword::Is;
use crate::lexer::Keyword::Not;
use crate::lexer::OperatorKind::*;
use crate::lexer::RawTokenKind::Keyword;
use crate::lexer::RawTokenKind::Operator;
use crate::parser::ast_node::ExprNode;
use crate::parser::ast_node::QualifiedOperator;
use crate::parser::ast_node::TypecastExpr;
use crate::parser::ast_node::UnaryExpr;
use crate::parser::combinators::keyword_if;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::expr_parsers::associativity::Associativity;
use crate::parser::expr_parsers::associativity::Associativity::Left;
use crate::parser::expr_parsers::associativity::Associativity::Non;
use crate::parser::expr_parsers::associativity::Associativity::Right;
use crate::parser::op_parsers::qual_op;
use crate::parser::result::Optional;
use crate::parser::result::Required;
use crate::parser::result::ScanResult;
use crate::parser::sign;
use crate::parser::token_stream::TokenConsumer;
use crate::parser::Parser;
