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

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        self.0.parse(stream)
            .required()
            .map_err(scan::Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::Error::ScanErr;
    use crate::tests::DEFAULT_CONFIG;
    use core::hint::unreachable_unchecked;
    use pg_elog::parser::Error::Syntax;
    use pg_lexer::Keyword;

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
        assert_eq!(&pg_elog::Error::Parser(Syntax), err.source())
    }

    #[test]
    fn test_eof() {
        let mut stream = TokenStream::new("", DEFAULT_CONFIG);
        let parser = required(Keyword::Precision);
        let actual = parser.parse(&mut stream);

        assert_matches!(actual, Err(ScanErr(_)));
        let ScanErr(err) = actual.unwrap_err() else { unsafe { unreachable_unchecked() } };
        assert_eq!(&pg_elog::Error::Parser(Syntax), err.source())
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::Required;
use crate::scan;
use crate::stream::TokenStream;
