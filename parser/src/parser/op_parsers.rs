impl<'src> Parser<'src> {

    /// Alias: `qual_Op`
    pub(super) fn qual_op(&mut self) -> ScanResult<QnOperator> {

        /*
            Operator | prefixed_operator
        */

        if let Some(op) = self.operator().no_match_to_option()? {
            let op = AllOp::Operator(op);
            return Ok(QnOperator(vec![], op))
        }

        self.prefixed_operator()
    }

    /// Production: `OPERATOR '(' any_operator ')'`
    pub(super) fn prefixed_operator(&mut self) -> ScanResult<QnOperator> {
        use crate::lexer::Keyword::Operator;
        use crate::lexer::TokenKind::{CloseParenthesis, OpenParenthesis};

        self.buffer.consume_kw_eq(Operator)?;

        self.buffer.consume_eq(OpenParenthesis).required()?;
        let op = self.any_operator().required()?;
        self.buffer.consume_eq(CloseParenthesis).required()?;

        Ok(op)
    }

    pub(super) fn any_operator(&mut self) -> ScanResult<QnOperator> {

        /*
            ( col_id '.' )* all_op
        */

        let mut qn = Vec::new();

        while let Some(id) = self.col_id().optional()? {
            self.buffer.consume_eq(Dot).required()?;
            qn.push(id);
        }

        let op = self.all_op();

        let op = if qn.is_empty() {
            op?
        }
        else {
            op.required()?
        };

        let op = QnOperator(qn, op);
        Ok(op)
    }

    /// Alias: `all_Op`
    pub(super) fn all_op(&mut self) -> ScanResult<AllOp> {

        if let Some(op) = self.math_op().no_match_to_option()? {
            return Ok(AllOp::MathOp(op))
        }

        self.operator().map(AllOp::Operator)
    }

    /// Returns the text of the `UserDefinedOperator`
    pub(super) fn operator(&mut self) -> ScanResult<String> {

        let loc = self.buffer.current_location();
        let source = self.buffer.source();
        self.buffer.consume(|tok| match tok {
            TokenKind::UserDefinedOperator => {
                let op = loc.slice(source).to_owned();
                Some(op)
            },
            _ => None
        })
    }

    /// Alias: `MathOp`
    pub(super) fn math_op(&mut self) -> ScanResult<MathOp> {
        use crate::parser::ast_node::MathOp::*;

        self.buffer.consume(|tok| match tok {
            TokenKind::Plus => Some(Addition),
            TokenKind::Minus => Some(Subtraction),
            TokenKind::Mul => Some(Multiplication),
            TokenKind::Div => Some(Division),
            TokenKind::Percent => Some(Modulo),
            TokenKind::Circumflex => Some(Exponentiation),
            TokenKind::Less => Some(Less),
            TokenKind::Greater => Some(Greater),
            TokenKind::Equals => Some(Equals),
            TokenKind::LessEquals => Some(LessEquals),
            TokenKind::GreaterEquals => Some(GreaterEquals),
            TokenKind::NotEquals => Some(NotEquals),
            _ => None
        })
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;


    #[test]
    fn test_qual_op() {
        use crate::parser::ast_node::{AllOp, QnOperator};

        let source = "operator(|/) <@>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = QnOperator(
            vec![],
            AllOp::Operator("|/".into())
        );
        assert_eq!(Ok(expected), parser.qual_op());

        let expected = QnOperator(
            vec![],
            AllOp::Operator("<@>".into())
        );
        assert_eq!(Ok(expected), parser.qual_op());
    }

    #[test]
    fn test_prefixed_operator() {
        use crate::parser::ast_node::{AllOp, MathOp, QnOperator};

        let source = "operator(some_qn.*)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.prefixed_operator();
        let expected = QnOperator(
            vec!["some_qn".into()],
            AllOp::MathOp(MathOp::Multiplication)
        );
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_any_operator() {
        use crate::parser::ast_node::{AllOp, MathOp, QnOperator};

        let source = "@@ != qn_name.+";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = QnOperator(
            vec![],
            AllOp::Operator("@@".into())
        );
        assert_eq!(Ok(expected), parser.any_operator());

        let expected = QnOperator(
            vec![],
            AllOp::MathOp(MathOp::NotEquals)
        );
        assert_eq!(Ok(expected), parser.any_operator());

        let expected = QnOperator(
            vec!["qn_name".into()],
            AllOp::MathOp(MathOp::Addition)
        );
        assert_eq!(Ok(expected), parser.any_operator());
    }

    #[test]
    fn test_all_op() {
        use AllOp::*;
        use crate::parser::ast_node::MathOp::NotEquals;

        let source = "~@ <>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Operator("~@".into())), parser.all_op());
        assert_eq!(Ok(MathOp(NotEquals)), parser.all_op());
    }

    #[test]
    fn test_operator() {
        let source = "~@";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("~@".into()), parser.operator());
    }

    #[test]
    fn test_math_op() {
        use MathOp::*;

        let source = "+ - * / % ^ < > = <= >= != <>";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Addition), parser.math_op());
        assert_eq!(Ok(Subtraction), parser.math_op());
        assert_eq!(Ok(Multiplication), parser.math_op());
        assert_eq!(Ok(Division), parser.math_op());
        assert_eq!(Ok(Modulo), parser.math_op());
        assert_eq!(Ok(Exponentiation), parser.math_op());
        assert_eq!(Ok(Less), parser.math_op());
        assert_eq!(Ok(Greater), parser.math_op());
        assert_eq!(Ok(Equals), parser.math_op());
        assert_eq!(Ok(LessEquals), parser.math_op());
        assert_eq!(Ok(GreaterEquals), parser.math_op());
        assert_eq!(Ok(NotEquals), parser.math_op());
        assert_eq!(Ok(NotEquals), parser.math_op());
    }
}

use crate::{
    lexer::TokenKind::{self, Dot},
    parser::{
        ast_node::{AllOp, MathOp, QnOperator},
        result::{Optional, Required, ScanResult, ScanResultTrait},
        token_buffer::TokenConsumer,
        Parser
    }
};
