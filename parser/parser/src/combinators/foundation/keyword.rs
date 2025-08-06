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

pub(in crate::combinators) fn any_keyword(stream: &mut TokenStream) -> scan::Result<Keyword> {
    stream.consume(|tok| match tok {
        TokenValue::Keyword(kw) => Some(*kw),
        _ => None
    })
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
    use crate::tests::test_parser;
    use pg_lexer::Keyword::Abort;
    use pg_lexer::KeywordCategory::Unreserved;

    #[test]
    fn test_keyword() {
        test_parser!(
            source = "abort",
            parser = Abort,
            expected = Abort
        )
    }

    #[test]
    fn test_keyword_category() {
        test_parser!(
            source = "abort",
            parser = Unreserved,
            expected = Abort
        )
    }

    #[test]
    fn test_keyword_if() {
        test_parser!(
            source = "abort",
            parser = keyword_if(|kw| kw.category() == Unreserved),
            expected = Abort
        )
    }
}

use crate::combinators::foundation::Combinator;
use crate::stream;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue;
use core::marker::PhantomData;
use pg_lexer::Keyword;
use pg_lexer::KeywordCategory;
use pg_parser_core::scan;
