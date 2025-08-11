impl Combinator for Keyword {
    type Output = Keyword;

    fn parse(&self, ctx: &mut ParserContext) -> scan::Result<Self::Output> {
        ctx.stream_mut().consume(|tok| match tok {
            TokenValue::Keyword(kw) if *kw == *self => Some(*kw),
            _ => None
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
impl Combinator for KeywordCategory {
    type Output = Keyword;

    fn parse(&self, ctx: &mut ParserContext) -> scan::Result<Self::Output> {
        ctx.stream_mut().consume(|tok| match tok {
            TokenValue::Keyword(kw) if kw.category() == *self => Some(*kw),
            _ => None
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_parser;
    use pg_lexer::Keyword::Abort;
    use pg_lexer::KeywordCategory::Unreserved;

    #[test]
    fn test_keyword() {
        test_parser!(
            source = "abort",
            parser = Abort,
            expected = Abort
        );
    }

    #[test]
    fn test_keyword_category() {
        test_parser!(
            source = "abort",
            parser = Unreserved,
            expected = Abort
        )
    }
}

use crate::Combinator;
use crate::ParserContext;
use pg_lexer::Keyword;
use pg_lexer::KeywordCategory;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenValue;
