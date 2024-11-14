impl Parser<'_> {
    /// Post-condition: Vec is **Not** empty
    pub(in crate::parser) fn indirection(&mut self) -> ScanResult<Vec<Indirection>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::indirection";

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
        const FN_NAME: &str = "postgres_parser::parser::Parser::indirection_el";

        /*
              '.' '*'
            | '.' ColLabel
            | '[' ':' ']'
            | '[' ':' a_expr ']'
            | '[' a_expr ']'
            | '[' a_expr ':' ']'
            | '[' a_expr ':' a_expr ']'
        */

        if self.buffer.consume_eq(Dot).no_match_to_option()?.is_some() {
            // `.`

            if self.buffer.consume_eq(Mul).try_match(fn_info!(FN_NAME))?.is_some() {
                // `.*`
                return Ok(Indirection::All)
            }

            let property = self.col_label().required(fn_info!(FN_NAME))?;
            // `.ColLabel`
            return Ok(Indirection::Property(property))
        }

        // `[`
        self.buffer.consume_eq(OpenBracket)?;

        if self.buffer.consume_eq(Colon).try_match(fn_info!(FN_NAME))?.is_some() {
            // `[ :`

            if self.buffer.consume_eq(CloseBracket).try_match(fn_info!(FN_NAME))?.is_some() {
                // `[ : ]`
                return Ok(Indirection::FullSlice)
            }

            // `[ : a_expr ]`
            let expr = self.a_expr().required(fn_info!(FN_NAME))?;
            self.buffer.consume_eq(CloseBracket).required(fn_info!(FN_NAME))?;

            return Ok(Indirection::SliceTo(expr))
        }

        // `[ a_expr`
        let left = self.a_expr().required(fn_info!(FN_NAME))?;

        if self.buffer.consume_eq(CloseBracket).try_match(fn_info!(FN_NAME))?.is_some() {
            // `[ a_expr ]`
            return Ok(Indirection::Index(left))
        }

        // `[ a_expr :`
        self.buffer.consume_eq(Colon).required(fn_info!(FN_NAME))?;

        if self.buffer.consume_eq(CloseBracket).try_match(fn_info!(FN_NAME))?.is_some() {
            // `[ a_expr : ]`
            return Ok(Indirection::SliceFrom(left))
        }

        // `[ expr : expr ]`
        let right = self.a_expr().required(fn_info!(FN_NAME))?;
        self.buffer.consume_eq(CloseBracket).required(fn_info!(FN_NAME))?;

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
    lexer::RawTokenKind::{CloseBracket, Colon, Dot, Mul, OpenBracket},
    parser::{
        ast_node::Indirection,
        result::{Optional, Required, ScanResult, ScanResultTrait, TryMatch},
        Parser
    },
};
use postgres_basics::fn_info;
