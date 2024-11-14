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

impl<'src> Parser<'src> {

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
        use RawTokenKind::{Equals, Greater, GreaterEquals, Keyword as Kw, Less, LessEquals, NotEquals};
        use crate::lexer::Keyword::{Ilike, Like, Operator as OperatorKw};

        const FN_NAME: &str = "postgres_parser::parser::Parser::operator";

        let slice = self.buffer.slice();

        consume! {self
            Ok {
                Plus if kind.intersects(OperatorKind::Additive) => Ok(Addition.into()),
                Minus if kind.intersects(OperatorKind::Additive) => Ok(Subtraction.into()),
                Mul if kind.intersects(OperatorKind::Multiplicative) => Ok(Multiplication.into()),
                Div if kind.intersects(OperatorKind::Multiplicative) => Ok(Division.into()),
                Percent if kind.intersects(OperatorKind::Multiplicative) => Ok(Modulo.into()),
                Circumflex if kind.intersects(OperatorKind::Exponentiation) => Ok(Exponentiation.into()),
                Less if kind.intersects(OperatorKind::Boolean) => Ok(Operator::Less.into()),
                Equals if kind.intersects(OperatorKind::Boolean) => Ok(Operator::Equals.into()),
                Greater if kind.intersects(OperatorKind::Boolean) => Ok(Operator::Greater.into()),
                LessEquals if kind.intersects(OperatorKind::Boolean) => Ok(Operator::LessEquals.into()),
                GreaterEquals if kind.intersects(OperatorKind::Boolean) => Ok(Operator::GreaterEquals.into()),
                NotEquals if kind.intersects(OperatorKind::Boolean) => Ok(Operator::NotEquals.into()),
                Kw(Like) if kind.intersects(OperatorKind::Like) => Ok(Operator::Like.into()),
                Kw(Ilike) if kind.intersects(OperatorKind::Like) => Ok(ILike.into()),
                UserDefinedOperator if kind.intersects(OperatorKind::UserDefined) => {
                    let op = slice.expect("slice is valid due to previous match").to_owned();
                    Ok(UserDefined(op).into())
                },
                Kw(OperatorKw) if kind.intersects(OperatorKind::Explicit) => {

                    /*
                        `OPERATOR '(' any_operator ')'`
                    */

                    self.buffer.consume_eq(OpenParenthesis).required(fn_info!(FN_NAME))?;
                    let op = self.any_operator().required(fn_info!(FN_NAME))?;
                    self.buffer.consume_eq(CloseParenthesis).required(fn_info!(FN_NAME))?;

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
        const FN_NAME: &str = "postgres_parser::parser::Parser::any_operator";

        /*
            ( col_id '.' )* all_op
        */

        let mut qn = Vec::new();

        while let Some(id) = self.col_id().optional()? {
            self.buffer.consume_eq(Dot).required(fn_info!(FN_NAME))?;
            qn.push(id);
        }

        let op = self.all_op();

        let op = if qn.is_empty() {
            op?
        }
        else {
            op.required(fn_info!(FN_NAME))?
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
            NotEquals
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
        assert_eq!(Ok(NotEquals), parser.all_op());
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
        assert_eq!(Ok(Less), parser.all_op());
        assert_eq!(Ok(Greater), parser.all_op());
        assert_eq!(Ok(Equals), parser.all_op());
        assert_eq!(Ok(LessEquals), parser.all_op());
        assert_eq!(Ok(GreaterEquals), parser.all_op());
        assert_eq!(Ok(NotEquals), parser.all_op());
        assert_eq!(Ok(NotEquals), parser.all_op());
    }

    #[test]
    fn test_subquery_op() {
        let source = "like ilike";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Like.into()), parser.subquery_op());
        assert_eq!(Ok(ILike.into()), parser.subquery_op());
    }
}

use crate::{
    lexer::RawTokenKind::{
        self,
        Circumflex,
        CloseParenthesis,
        Div,
        Dot,
        Minus,
        Mul,
        OpenParenthesis,
        Percent,
        Plus,
        UserDefinedOperator,
    },
    parser::{
        ast_node::{
            Operator::{self, *},
            QualifiedOperator
        },
        consume_macro::consume,
        result::{Optional, Required, ScanErrorKind::NoMatch, ScanResult},
        Parser
    }
};
use bitflags::bitflags;
use postgres_basics::fn_info;
