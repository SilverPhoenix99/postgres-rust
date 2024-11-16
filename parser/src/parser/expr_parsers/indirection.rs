impl Parser<'_> {
    /// Post-condition: Vec is **Not** empty
    pub(in crate::parser) fn indirection(&mut self) -> ScanResult<Vec<Indirection>> {

        /*
            ( indirection_el )+
        */

        let mut elements = vec![self.indirection_el()?];

        while let Some(element) = self.indirection_el().optional()? {
            elements.push(element);
        }

        Ok(elements)
    }

    fn indirection_el(&mut self) -> ScanResult<Indirection> {

        /*
              '.' '*'
            | '.' ColLabel
            | '[' ':' ']'
            | '[' ':' a_expr ']'
            | '[' a_expr ']'
            | '[' a_expr ':' ']'
            | '[' a_expr ':' a_expr ']'
        */

        if self.buffer.consume_op(Dot).no_match_to_option()?.is_some() {
            // `.`

            if self.buffer.consume_op(Mul).try_match(fn_info!())?.is_some() {
                // `.*`
                return Ok(Indirection::All)
            }

            let property = self.col_label().required(fn_info!())?;
            // `.ColLabel`
            return Ok(Indirection::Property(property))
        }

        // `[`
        self.buffer.consume_op(OpenBracket)?;

        if self.buffer.consume_op(Colon).try_match(fn_info!())?.is_some() {
            // `[ :`

            if self.buffer.consume_op(CloseBracket).try_match(fn_info!())?.is_some() {
                // `[ : ]`
                return Ok(Indirection::FullSlice)
            }

            // `[ : a_expr ]`
            let expr = self.a_expr().required(fn_info!())?;
            self.buffer.consume_op(CloseBracket).required(fn_info!())?;

            return Ok(Indirection::SliceTo(expr))
        }

        // `[ a_expr`
        let left = self.a_expr().required(fn_info!())?;

        if self.buffer.consume_op(CloseBracket).try_match(fn_info!())?.is_some() {
            // `[ a_expr ]`
            return Ok(Indirection::Index(left))
        }

        // `[ a_expr :`
        self.buffer.consume_op(Colon).required(fn_info!())?;

        if self.buffer.consume_op(CloseBracket).try_match(fn_info!())?.is_some() {
            // `[ a_expr : ]`
            return Ok(Indirection::SliceFrom(left))
        }

        // `[ expr : expr ]`
        let right = self.a_expr().required(fn_info!())?;
        self.buffer.consume_op(CloseBracket).required(fn_info!())?;

        Ok(Indirection::Slice(left, right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use Indirection::*;

    #[test]
    fn test_indirection() {
        let mut parser = Parser::new(".some_property.*[:]", DEFAULT_CONFIG);

        let expected = vec![
            Property("some_property".into()),
            All,
            FullSlice,
            // TODO Index(_),
            // TODO SliceFrom(_),
            // TODO SliceTo(_),
            // TODO Slice(_, _)
        ];

        assert_eq!(Ok(expected), parser.indirection());
    }
}

use crate::{
    lexer::OperatorKind::{CloseBracket, Colon, Dot, Mul, OpenBracket},
    parser::{
        ast_node::Indirection,
        result::{Optional, Required, ScanResult, ScanResultTrait, TryMatch},
        Parser
    },
};
use postgres_basics::fn_info;
