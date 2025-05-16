/// `Eof` and `NoMatch` become `Err(Syntax)`.
pub(in crate::combinators) fn required<P>(parser: P) -> RequiredCombi<P>
where
    P: Combinator
{
    RequiredCombi(parser)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct RequiredCombi<P>(P);

impl<P> Combinator for RequiredCombi<P>
where
    P: Combinator
{
    type Output = P::Output;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        self.0.parse(stream)
            .required()
            .map_err(ScanErrorKind::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::ScanErrorKind::ScanErr;
    use crate::tests::DEFAULT_CONFIG;
    use pg_elog::parser::ParserErrorKind;
    use pg_lexer::Keyword;
    use std::hint::unreachable_unchecked;

    #[test]
    fn test_required() {
        let mut stream = TokenStream::new("precision", DEFAULT_CONFIG);
        let parser = required(Keyword::Precision);
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(Keyword::Precision), actual);
    }

    #[test]
    fn test_no_match() {
        let mut stream = TokenStream::new("abort", DEFAULT_CONFIG);
        let parser = required(Keyword::Precision);
        let actual = parser.parse(&mut stream);

        assert_matches!(actual, Err(ScanErr(_)));
        let ScanErr(err) = actual.unwrap_err() else { unsafe { unreachable_unchecked() } };
        assert_eq!(&ParserErrorKind::Syntax, err.source())
    }

    #[test]
    fn test_eof() {
        let mut stream = TokenStream::new("", DEFAULT_CONFIG);
        let parser = required(Keyword::Precision);
        let actual = parser.parse(&mut stream);

        assert_matches!(actual, Err(ScanErr(_)));
        let ScanErr(err) = actual.unwrap_err() else { unsafe { unreachable_unchecked() } };
        assert_eq!(&ParserErrorKind::Syntax, err.source())
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::Required;
use crate::result::ScanErrorKind;
use crate::result::ScanResult;
use crate::stream::TokenStream;
