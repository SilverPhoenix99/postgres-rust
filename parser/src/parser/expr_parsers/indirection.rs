impl Parser<'_> {
    /// Post-condition: Vec is **Not** empty
    pub(in crate::parser) fn indirection(&mut self) -> ScanResult<Vec<Indirection>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::indirection";

        let mut elements = Vec::new();

        while !self.buffer.eof() {

            if self.buffer.consume_eq(Dot).optional()?.is_some() {

                if self.buffer.consume_eq(Mul).try_match(fn_info!(FN_NAME))?.is_some() {
                    // `.*`
                    elements.push(Indirection::All);
                    continue
                }

                if let Some(property) = self.col_label().try_match(fn_info!(FN_NAME))? {
                    // `.ColLabel`
                    elements.push(Indirection::Property(property));
                    continue
                }

                return Err(syntax_err!(FN_NAME))
            }

            if self.buffer.consume_eq(OpenBracket).optional()?.is_some() {

                if self.buffer.consume_eq(Colon).try_match(fn_info!(FN_NAME))?.is_some() {

                    if self.buffer.consume_eq(CloseBracket).try_match(fn_info!(FN_NAME))?.is_some() {
                        // `[ : ]`
                        elements.push(Indirection::FullSlice);
                        continue
                    }

                    // `[ : expr ]`
                    let expr = self.a_expr().required(fn_info!(FN_NAME))?;
                    elements.push(Indirection::SliceTo(expr));
                    continue
                }

                let left = self.a_expr().required(fn_info!(FN_NAME))?;

                if self.buffer.consume_eq(CloseBracket).try_match(fn_info!(FN_NAME))?.is_some() {
                    // `[ expr ]`
                    elements.push(Indirection::Index(left));
                    continue
                }

                if self.buffer.consume_eq(Colon).try_match(fn_info!(FN_NAME))?.is_some() {

                    if self.buffer.consume_eq(CloseBracket).try_match(fn_info!(FN_NAME))?.is_some() {
                        // `[ expr : ]`
                        elements.push(Indirection::SliceFrom(left));
                        continue
                    }

                    // `[ expr : expr ]`
                    let right = self.a_expr().required(fn_info!(FN_NAME))?;
                    elements.push(Indirection::Slice(left, right));
                    continue
                }

                return Err(syntax_err!(FN_NAME))
            }

            break
        }

        if elements.is_empty() {
            return Err(NoMatch)
        }

        Ok(elements)
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
    lexer::TokenKind::{CloseBracket, Colon, Dot, Mul, OpenBracket},
    parser::{
        ast_node::Indirection,
        error::syntax_err,
        result::ScanErrorKind::NoMatch,
        result::{Optional, Required, ScanResult, TryMatch},
        Parser,
    }
};
use postgres_basics::fn_info;
