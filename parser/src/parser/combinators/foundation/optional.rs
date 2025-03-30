/// `Eof` and `NoMatch` become `Ok(None)`.
pub(in crate::parser::combinators) fn optional<P>(parser: P) -> OptionalCombi<P>
where
    P: Combinator
{
    OptionalCombi(parser)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser::combinators) struct OptionalCombi<P>(P);

impl<P> OptionalCombi<P> {
    fn optional(self) -> OptionalCombi<P> {
        self
    }
}

impl<P> Combinator for OptionalCombi<P>
where
    P: Combinator
{
    type Output = Option<P::Output>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        self.0.parse(stream)
            .optional()
            .map_err(ScanErrorKind::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Keyword;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_optional() {
        let mut stream = TokenStream::new("precision", DEFAULT_CONFIG);
        let parser = optional(Keyword::Precision);
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(Some(Keyword::Precision)), actual);
    }

    #[test]
    fn test_optional_no_match() {
        let mut stream = TokenStream::new("abort", DEFAULT_CONFIG);
        let parser = optional(Keyword::Precision);
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(None), actual);
    }

    #[test]
    fn test_optional_eof() {
        let mut stream = TokenStream::new("", DEFAULT_CONFIG);
        let parser = optional(Keyword::Precision);
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(None), actual);
    }
}

use crate::parser::combinators::foundation::Combinator;
use crate::parser::result::Optional as Opt;
use crate::parser::result::ScanErrorKind;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::TokenStream;
