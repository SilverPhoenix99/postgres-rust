pub(in crate::parser) fn keyword(keyword: Keyword) -> KeywordCombi {
    KeywordCombi(keyword)
}

pub(in crate::parser) fn keyword_category(category: KeywordCategory) -> KeywordCategoryCombi {
    KeywordCategoryCombi(category)
}

/// Conditionally consumes the keyword.
///
/// * If the `mapper` returns `true`, then the keyword is consumed.
/// * Otherwise, when `false` is returned, then an `Err(NoMatch)` is emitted and the keyword is **Not** consumed.
///
/// See also
/// * [`keyword_result()`]
/// * [`keyword_when()`]
pub(in crate::parser) fn keyword_if<P>(pred: P)
    -> KeywordCondCombi<
        impl Fn(Keyword) -> ConsumerResult<Keyword>,
        Keyword
    >
where
    P: Fn(Keyword) -> bool
{
    keyword_result(move |kw| Ok(pred(kw).then_some(kw)))
}

/// Maps the keyword before consuming it.
///
/// * If the `mapper` returns `Some(_)`, then the keyword is consumed.
/// * Otherwise, when `None` is returned, then an `Err(NoMatch)` is emitted and the keyword is **Not** consumed.
///
/// See also
/// * [`keyword_result()`]
/// * [`keyword_if()`]
pub(in crate::parser) fn keyword_when<O>(
    mapper: impl Fn(Keyword) -> Option<O>
)
    -> KeywordCondCombi<
        impl Fn(Keyword) -> ConsumerResult<O>,
        O
    >
{
    keyword_result(move |kw| Ok(mapper(kw)))
}

/// Maps the keyword before consuming it.
///
/// * If the `mapper` returns `Ok(Some(_))`, then the keyword is consumed.
/// * If it returns, `Ok(None)`, then an `Err(NoMatch)` is emitted and the keyword is **Not** consumed.
/// * [`ParserError`](crate::parser::error::ParserError) can be returned to stop the parser.
///   For example, if the keyword is illegal.
///
/// See also
/// * [`keyword_when()`]
/// * [`keyword_if()`]
pub(in crate::parser) fn keyword_result<O>(
    mapper: impl Fn(Keyword) -> ConsumerResult<O>
) -> KeywordCondCombi<
        impl Fn(Keyword) -> ConsumerResult<O>,
        O
    >
{
    KeywordCondCombi {
        mapper,
        boo: PhantomData,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct KeywordCombi(Keyword);

impl Combinator for KeywordCombi {
    type Output = Keyword;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok|
            tok.keyword().filter(|kw|
                kw == &self.0
            )
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct KeywordCategoryCombi(KeywordCategory);

impl Combinator for KeywordCategoryCombi {
    type Output = Keyword;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok|
            tok.keyword().filter(|kw|
                kw.details().category() == self.0
            )
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct KeywordCondCombi<F, O> {
    mapper: F,
    boo: PhantomData<O>
}

impl<F, O> Combinator for KeywordCondCombi<F, O>
where
    F: Fn(Keyword) -> ConsumerResult<O>
{
    type Output = O;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok| {
            match tok.keyword() {
                Some(kw) => (self.mapper)(kw),
                None => Ok(None)
            }
        })
    }
}

impl<F, O> Debug for KeywordCondCombi<F, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("KeywordCondCombi")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::KeywordCategory::Unreserved;
    use crate::parser::result::ScanErrorKind::{Eof, NoMatch};
    use crate::parser::tests::DEFAULT_CONFIG;
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
use crate::parser::combinators::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::{ConsumerResult, TokenConsumer, TokenStream};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
