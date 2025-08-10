/// `Eof` and `NoMatch` become `Ok(None)`.
pub fn optional<P>(parser: P) -> OptionalCombi<P>
where
    P: Combinator
{
    OptionalCombi(parser)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OptionalCombi<P>(P);

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
    use test_case::test_case;

    #[test_case("precision", Some(Keyword::Precision))]
    #[test_case("abort", None)]
    #[test_case("", None)]
    fn test_optional(source: &str, expected: Option<Keyword>) {
        test_parser!(source, optional(Keyword::Precision), expected)
    }
}

use crate::Combinator;
use crate::ParserContext;
use pg_parser_core::scan;
use pg_parser_core::Optional as Opt;
