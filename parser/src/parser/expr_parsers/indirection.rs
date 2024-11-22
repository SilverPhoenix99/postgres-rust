use crate::parser::combinators::{operator, or, Combinator};
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

        let result = operator(Dot)
            .and_right(
                or(
                    // `.*`
                    operator(Mul).map(|_| Indirection::All),
                    // `.ColLabel`
                    col_label().map(Indirection::Property)
                )
            )
            .maybe_match()
            .parse(&mut self.buffer)?;

        if let Some(indirection) = result {
            return Ok(indirection)
        }

        // `[`
        operator(OpenBracket).parse(&mut self.buffer)?;

        if operator(Colon).try_match().parse(&mut self.buffer)?.is_some() {
            // `[ :`

            if operator(CloseBracket).try_match().parse(&mut self.buffer)?.is_some() {
                // `[ : ]`
                return Ok(Indirection::FullSlice)
            }

            // `[ : a_expr ]`
            let expr = self.a_expr().required()?;
            operator(CloseBracket).required().parse(&mut self.buffer)?;

            return Ok(Indirection::SliceTo(expr))
        }

        // `[ a_expr`
        let left = self.a_expr().required()?;

        if operator(CloseBracket).try_match().parse(&mut self.buffer)?.is_some() {
            // `[ a_expr ]`
            return Ok(Indirection::Index(left))
        }

        // `[ a_expr :`
        operator(Colon).required().parse(&mut self.buffer)?;

        if operator(CloseBracket).try_match().parse(&mut self.buffer)?.is_some() {
            // `[ a_expr : ]`
            return Ok(Indirection::SliceFrom(left))
        }

        // `[ a_expr : a_expr ]`
        let right = self.a_expr().required()?;
        operator(CloseBracket).required().parse(&mut self.buffer)?;

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

use crate::lexer::OperatorKind::CloseBracket;
use crate::lexer::OperatorKind::Colon;
use crate::lexer::OperatorKind::Dot;
use crate::lexer::OperatorKind::Mul;
use crate::lexer::OperatorKind::OpenBracket;
use crate::parser::ast_node::Indirection;
use crate::parser::col_label;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::result::Optional;
use crate::parser::result::Required;
use crate::parser::result::ScanResult;
use crate::parser::Parser;
