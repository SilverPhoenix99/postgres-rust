/// Conditionally consumes the keyword.
///
/// * If the `mapper` returns `true`, then the keyword is consumed.
/// * Otherwise, when `false` is returned, then an `Err(NoMatch)` is emitted and the keyword is **Not** consumed.
///
/// See also
/// * [`keyword_result()`]
/// * [`keyword_when()`]
pub(in crate::combinators) fn keyword_if<P>(pred: P)
    -> KeywordCondCombi<
        impl Fn(Keyword) -> stream::LocatedResult<Keyword>,
        Keyword
    >
where
    P: Fn(Keyword) -> bool
{
    keyword_result(move |kw| Ok(pred(kw).then_some(kw)))
}

pub(in crate::combinators) fn any_keyword()
    -> KeywordCondCombi<
        impl Fn(Keyword) -> stream::LocatedResult<Keyword>,
        Keyword
    >
{
    keyword_if(|_| true)
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
pub(in crate::combinators) fn keyword_result<O>(
    mapper: impl Fn(Keyword) -> stream::LocatedResult<O>
) -> KeywordCondCombi<
        impl Fn(Keyword) -> stream::LocatedResult<O>,
        O
    >
{
    KeywordCondCombi {
        mapper,
        boo: PhantomData,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
impl Combinator for Keyword {
    type Output = Keyword;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        stream.consume(|tok| match tok {
            TokenValue::Keyword(kw) if *kw == *self => Some(*kw),
            _ => None
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
impl Combinator for KeywordCategory {
    type Output = Keyword;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        stream.consume(|tok| match tok {
            TokenValue::Keyword(kw) if kw.category() == *self => Some(*kw),
            _ => None
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct KeywordCondCombi<F, O> {
    mapper: F,
    boo: PhantomData<O>
}

impl<F, O> Combinator for KeywordCondCombi<F, O>
where
    F: Fn(Keyword) -> stream::LocatedResult<O>
{
    type Output = O;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        stream.consume(|tok| {
            match tok {
                TokenValue::Keyword(kw) => (self.mapper)(*kw),
                _ => Ok(None)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::DEFAULT_CONFIG;
    use pg_lexer::Keyword::Abort;
    use pg_lexer::KeywordCategory::Unreserved;

    #[test]
    fn test_keyword() {
        let mut stream = TokenStream::new("abort", DEFAULT_CONFIG);
        let actual = Abort.parse(&mut stream);
        assert_eq!(Ok(Abort), actual);
    }

    #[test]
    fn test_keyword_category() {
        let mut stream = TokenStream::new("abort", DEFAULT_CONFIG);
        let actual = Unreserved.parse(&mut stream);
        assert_eq!(Ok(Abort), actual);
    }

    #[test]
    fn test_keyword_if() {
        let mut stream = TokenStream::new("abort", DEFAULT_CONFIG);
        let parser = keyword_if(|kw| kw.category() == Unreserved);
        let actual = parser.parse(&mut stream);
        assert_eq!(Ok(Abort), actual);
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue;
use core::marker::PhantomData;
use pg_lexer::Keyword;
use pg_lexer::KeywordCategory;
