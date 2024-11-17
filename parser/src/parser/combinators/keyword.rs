pub(in crate::parser) fn keyword(kw: Keyword) -> KeywordCombi<impl Fn(Keyword) -> bool> {
    keyword_if(move |tok| tok == kw)
}

pub(in crate::parser) fn keyword_if<P>(pred: P) -> KeywordCombi<P>
where
    P: Fn(Keyword) -> bool
{
    KeywordCombi { pred }
}

pub(in crate::parser) fn any_keyword() -> KeywordCombi<impl Fn(Keyword) -> bool> {
    keyword_if(|_| true)
}

pub(in crate::parser) fn keyword_category(category: KeywordCategory) -> KeywordCombi<impl Fn(Keyword) -> bool> {
    keyword_if(move |tok| tok.details().category() == category)
}

pub(in crate::parser) struct KeywordCombi<P>
where
    P: Fn(Keyword) -> bool
{
    pred: P
}

impl<P> ParserFunc for KeywordCombi<P>
where
    P: Fn(Keyword) -> bool
{
    type Output = Keyword;
    type Error = ScanErrorKind;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok|
            tok.keyword().filter(|kw| (self.pred)(*kw))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::KeywordCategory::Unreserved;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::ScanErrorKind::*;
    use postgres_basics::Location;

    #[test]
    fn test_keyword() {
        let mut stream = TokenStream::new("precision", DEFAULT_CONFIG);
        let parser = keyword(Keyword::Precision);
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(Keyword::Precision), actual);
    }

    #[test]
    fn test_keyword_no_match() {
        let mut stream = TokenStream::new("precision", DEFAULT_CONFIG);
        let parser = keyword(Keyword::Varchar);
        let actual = parser.parse(&mut stream);
        assert_eq!(Err(NoMatch(Location::new(0..9, 1, 1))), actual);
    }

    #[test]
    fn test_keyword_eof() {
        let mut stream = TokenStream::new("", DEFAULT_CONFIG);
        let parser = keyword(Keyword::Precision);
        let actual = parser.parse(&mut stream);
        assert_eq!(Err(Eof(Location::new(0..0, 1, 1))), actual);
    }

    #[test]
    fn test_keyword_if() {
        let mut stream = TokenStream::new("abort", DEFAULT_CONFIG);
        let parser = keyword_if(|kw| kw.details().category() == Unreserved);
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(Keyword::Abort), actual);
    }

    #[test]
    fn test_keyword_category() {
        let mut stream = TokenStream::new("abort", DEFAULT_CONFIG);
        let parser = keyword_category(Unreserved);
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(Keyword::Abort), actual);
    }
}

use crate::lexer::{Keyword, KeywordCategory};
use crate::parser::combinators::ParserFunc;
use crate::parser::result::{ScanErrorKind, ScanResult};
use crate::parser::token_stream::{TokenConsumer, TokenStream};
