pub(in crate::parser) fn optional<P>(parser: P) -> OptionalCombi<P>
where
    P: ParserFunc
{
    OptionalCombi { parser }
}

pub(in crate::parser) struct OptionalCombi<P>
where
    P: ParserFunc
{
    parser: P
}

impl<P> ParserFunc for OptionalCombi<P>
where
    P: ParserFunc,
    ScanErrorKind: From<P::Error>
{
    type Output = Option<P::Output>;
    type Error = ParserError;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ParseResult<Self::Output> {
        self.parser.parse(stream)
            .map_err(ScanErrorKind::from)
            .optional()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Keyword;
    use crate::parser::combinators::keyword;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_optional() {
        let mut stream = TokenStream::new("precision", DEFAULT_CONFIG);
        let parser = optional(keyword(Keyword::Precision));
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(Some(Keyword::Precision)), actual);
    }

    #[test]
    fn test_optional_no_match() {
        let mut stream = TokenStream::new("abort", DEFAULT_CONFIG);
        let parser = optional(keyword(Keyword::Precision));
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(None), actual);
    }

    #[test]
    fn test_optional_eof() {
        let mut stream = TokenStream::new("", DEFAULT_CONFIG);
        let parser = optional(keyword(Keyword::Precision));
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(None), actual);
    }
}

use crate::parser::combinators::ParserFunc;
use crate::parser::result::{Optional as Opt, ScanErrorKind};
use crate::parser::token_stream::TokenStream;
use crate::parser::{ParseResult, ParserError};
