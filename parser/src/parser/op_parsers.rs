bitflags! {
  pub(in super) struct OperatorKind : u8 {
    const Additive       = 1;
    const Multiplicative = 1 << 1;
    const Exponentiation = 1 << 2;
    const Boolean        = 1 << 3;
    const Explicit       = 1 << 4;
    const UserDefined    = 1 << 5;
    const Like           = 1 << 6;

    const Math = Self::Additive.bits()
               | Self::Multiplicative.bits()
               | Self::Exponentiation.bits()
               | Self::Boolean.bits();

    const Qualified = Self::UserDefined.bits() | Self::Explicit.bits();
    const Unqualified = Self::UserDefined.bits() | Self::Math.bits();
    const All = Self::Unqualified.bits() | Self::Qualified.bits();
    const Subquery = Self::All.bits() | Self::Like.bits();
  }
}

impl Parser<'_> {

    /// Alias: `qual_Op`
    pub(super) fn qual_op(&mut self) -> ScanResult<QualifiedOperator> {
        self.operator(OperatorKind::Qualified)
    }

    /// Alias: `all_Op`
    pub(super) fn all_op(&mut self) -> ScanResult<Operator> {
        let QualifiedOperator(_qo, op) = self.operator(OperatorKind::Unqualified)?;
        debug_assert!(_qo.is_empty());
        Ok(op)
    }

    /// Alias: `qual_all_Op`
    pub(super) fn qual_all_op(&mut self) -> ScanResult<QualifiedOperator> {
        self.operator(OperatorKind::All)
    }

    /// Alias: `subquery_Op`
    pub(super) fn subquery_op(&mut self) -> ScanResult<QualifiedOperator> {
        self.operator(OperatorKind::Subquery)
    }

    pub(super) fn operator(&mut self, kind: OperatorKind) -> ScanResult<QualifiedOperator> {

        let slice = self.buffer.slice();

        consume! {self
            Ok {
                Op(Plus) if kind.intersects(OperatorKind::Additive) => Ok(Addition.into()),
                Op(Minus) if kind.intersects(OperatorKind::Additive) => Ok(Subtraction.into()),
                Op(Mul) if kind.intersects(OperatorKind::Multiplicative) => Ok(Multiplication.into()),
                Op(Div) if kind.intersects(OperatorKind::Multiplicative) => Ok(Division.into()),
                Op(Percent) if kind.intersects(OperatorKind::Multiplicative) => Ok(Modulo.into()),
                Op(Circumflex) if kind.intersects(OperatorKind::Exponentiation) => Ok(Exponentiation.into()),
                Op(Less) if kind.intersects(OperatorKind::Boolean) => Ok(Operator::Less.into()),
                Op(Equals) if kind.intersects(OperatorKind::Boolean) => Ok(Operator::Equals.into()),
                Op(Greater) if kind.intersects(OperatorKind::Boolean) => Ok(Operator::Greater.into()),
                Op(LessEquals) if kind.intersects(OperatorKind::Boolean) => Ok(Operator::LessEquals.into()),
                Op(GreaterEquals) if kind.intersects(OperatorKind::Boolean) => Ok(Operator::GreaterEquals.into()),
                Op(NotEquals) if kind.intersects(OperatorKind::Boolean) => Ok(Operator::NotEquals.into()),
                Kw(Like) if kind.intersects(OperatorKind::Like) => Ok(Operator::Like.into()),
                Kw(Ilike) if kind.intersects(OperatorKind::Like) => Ok(ILike.into()),
                UserDefinedOperator if kind.intersects(OperatorKind::UserDefined) => {
                    let op = slice.expect("slice is valid due to previous match").into();
                    Ok(UserDefined(op).into())
                },
                Kw(OperatorKw) if kind.intersects(OperatorKind::Explicit) => {

                    /*
                        `OPERATOR '(' any_operator ')'`
                    */

                    OpenParenthesis.required().parse(&mut self.buffer)?;
                    let op = self.any_operator().required()?;
                    CloseParenthesis.required().parse(&mut self.buffer)?;

                    Ok(op)
                },
            }
            Err {
                Ok(_) => {
                    let loc = self.buffer.current_location();
                    NoMatch(loc)
                },
                Err(err) => err.into(),
            }
        }
    }

    pub(super) fn any_operator(&mut self) -> ScanResult<QualifiedOperator> {

        /*
            ( col_id '.' )* all_op
        */

        let qn = many(col_id().and_left(Dot))
            .optional()
            .parse(&mut self.buffer)?
            .unwrap_or_default();

        let op = self.all_op();

        let op = if qn.is_empty() {
            op?
        }
        else {
            op.required()?
        };

        let op = QualifiedOperator(qn, op);
        Ok(op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_user_defined_op() {

        let source = "operator(|/) <@>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = QualifiedOperator(vec![], UserDefined("|/".into()));
        assert_eq!(Ok(expected), parser.qual_op());

        let expected = QualifiedOperator(vec![], UserDefined("<@>".into()));
        assert_eq!(Ok(expected), parser.qual_op());
    }

    #[test]
    fn test_qualified_op() {
        let source = "operator(some_qn.*)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.qual_op();
        let expected = QualifiedOperator(
            vec!["some_qn".into()],
            Multiplication
        );
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_any_operator() {
        let source = "@@ != q_name.+";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = QualifiedOperator(
            vec![],
            UserDefined("@@".into())
        );
        assert_eq!(Ok(expected), parser.any_operator());

        let expected = QualifiedOperator(
            vec![],
            Operator::NotEquals
        );
        assert_eq!(Ok(expected), parser.any_operator());

        let expected = QualifiedOperator(
            vec!["q_name".into()],
            Addition
        );
        assert_eq!(Ok(expected), parser.any_operator());
    }

    #[test]
    fn test_all_op() {
        let source = "~@ <>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(UserDefined("~@".into())), parser.all_op());
        assert_eq!(Ok(Operator::NotEquals), parser.all_op());
    }

    #[test]
    fn test_math_op() {

        let source = "+ - * / % ^ < > = <= >= != <>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Addition), parser.all_op());
        assert_eq!(Ok(Subtraction), parser.all_op());
        assert_eq!(Ok(Multiplication), parser.all_op());
        assert_eq!(Ok(Division), parser.all_op());
        assert_eq!(Ok(Modulo), parser.all_op());
        assert_eq!(Ok(Exponentiation), parser.all_op());
        assert_eq!(Ok(Operator::Less), parser.all_op());
        assert_eq!(Ok(Operator::Greater), parser.all_op());
        assert_eq!(Ok(Operator::Equals), parser.all_op());
        assert_eq!(Ok(Operator::LessEquals), parser.all_op());
        assert_eq!(Ok(Operator::GreaterEquals), parser.all_op());
        assert_eq!(Ok(Operator::NotEquals), parser.all_op());
        assert_eq!(Ok(Operator::NotEquals), parser.all_op());
    }

    #[test]
    fn test_subquery_op() {
        let source = "like ilike";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Operator::Like.into()), parser.subquery_op());
        assert_eq!(Ok(ILike.into()), parser.subquery_op());
    }
}

use crate::lexer::Keyword::Ilike;
use crate::lexer::Keyword::Like;
use crate::lexer::Keyword::Operator as OperatorKw;
use crate::lexer::OperatorKind::Circumflex;
use crate::lexer::OperatorKind::CloseParenthesis;
use crate::lexer::OperatorKind::Div;
use crate::lexer::OperatorKind::Dot;
use crate::lexer::OperatorKind::Equals;
use crate::lexer::OperatorKind::Greater;
use crate::lexer::OperatorKind::GreaterEquals;
use crate::lexer::OperatorKind::Less;
use crate::lexer::OperatorKind::LessEquals;
use crate::lexer::OperatorKind::Minus;
use crate::lexer::OperatorKind::Mul;
use crate::lexer::OperatorKind::NotEquals;
use crate::lexer::OperatorKind::OpenParenthesis;
use crate::lexer::OperatorKind::Percent;
use crate::lexer::OperatorKind::Plus;
use crate::lexer::RawTokenKind::Keyword as Kw;
use crate::lexer::RawTokenKind::Operator as Op;
use crate::lexer::RawTokenKind::UserDefinedOperator;
use crate::parser::ast_node::Operator;
use crate::parser::ast_node::Operator::Addition;
use crate::parser::ast_node::Operator::Division;
use crate::parser::ast_node::Operator::Exponentiation;
use crate::parser::ast_node::Operator::ILike;
use crate::parser::ast_node::Operator::Modulo;
use crate::parser::ast_node::Operator::Multiplication;
use crate::parser::ast_node::Operator::Subtraction;
use crate::parser::ast_node::Operator::UserDefined;
use crate::parser::ast_node::QualifiedOperator;
use crate::parser::col_id;
use crate::parser::combinators::many;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::consume_macro::consume;
use crate::parser::result::Required;
use crate::parser::result::ScanErrorKind::NoMatch;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
use bitflags::bitflags;
