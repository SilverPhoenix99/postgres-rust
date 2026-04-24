/// `Eof` and `NoMatch` become `Ok(None)`.
pub(in crate::combinators) fn optional<P>(parser: P) -> OptionalCombi<P>
where
    P: Combinator
{
    OptionalCombi(parser)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct OptionalCombi<P>(P);

impl<P> Combinator for OptionalCombi<P>
where
    P: Combinator
{
    type Output = Option<P::Output>;

    fn parse(&self, ctx: &mut ParserContext) -> scan::Result<Self::Output> {
        self.0.parse(ctx)
            .optional()
            .map_err(scan::Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_lexer::Keyword;
    use test_case::test_matrix;

    #[test_matrix("precision" => Ok(Some(Keyword::Precision)))]
    #[test_matrix("abort" => Ok(None))]
    #[test_matrix("" => Ok(None))]
    fn test_optional(source: &str) -> scan::Result<Option<Keyword>> {
        test_parser!(source, optional(Keyword::Precision))
    }
}

use crate::combinators::core::Combinator;
use crate::ParserContext;
use pg_parser_core::scan;
use pg_parser_core::Optional as Opt;
