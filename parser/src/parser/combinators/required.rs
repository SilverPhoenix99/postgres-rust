pub(in crate::parser) fn required<P>(parser: P, caller: &'static FnInfo) -> RequiredCombi<P>
where
    P: ParserFunc
{
    RequiredCombi { parser, caller }
}

pub(in crate::parser) struct RequiredCombi<P>
where
    P: ParserFunc
{
    parser: P,
    caller: &'static FnInfo,
}

impl<P> ParserFunc for RequiredCombi<P>
where
    P: ParserFunc,
    ScanErrorKind: From<P::Error>
{
    type Output = P::Output;
    type Error = ParserError;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ParseResult<Self::Output> {
        self.parser.parse(stream)
            .map_err(ScanErrorKind::from)
            .required(self.caller)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Keyword;
    use crate::parser::combinators::keyword;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::ParserErrorKind;
    use postgres_basics::fn_info;

    #[test]
    fn test_required() {
        let mut stream = TokenStream::new("precision", DEFAULT_CONFIG);
        let parser = required(keyword(Keyword::Precision), fn_info!("test_required"));
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(Keyword::Precision), actual);
    }

    #[test]
    fn test_no_match() {
        let mut stream = TokenStream::new("abort", DEFAULT_CONFIG);
        let parser = required(keyword(Keyword::Precision), fn_info!("test_required"));
        let actual = parser.parse(&mut stream);

        assert_matches!(actual, Err(_));
        let err = actual.unwrap_err();
        assert_eq!(&ParserErrorKind::Syntax, err.source())
    }

    #[test]
    fn test_eof() {
        let mut stream = TokenStream::new("", DEFAULT_CONFIG);
        let parser = required(keyword(Keyword::Precision), fn_info!("test_required"));
        let actual = parser.parse(&mut stream);

        assert_matches!(actual, Err(_));
        let err = actual.unwrap_err();
        assert_eq!(&ParserErrorKind::Syntax, err.source())
    }
}

use crate::parser::combinators::ParserFunc;
use crate::parser::result::{Required, ScanErrorKind};
use crate::parser::token_stream::TokenStream;
use crate::parser::{ParseResult, ParserError};
use postgres_basics::FnInfo;
