/// Aliases:
/// * `ColLabel`
/// * `attr_name`
pub fn col_label(ctx: &mut ParserContext) -> scan::Result<Str> {

    alt!(
        identifier.map(From::from),
        any_keyword.map(From::from)
    ).parse(ctx)
}

fn any_keyword(ctx: &mut ParserContext) -> scan::Result<Keyword> {
    ctx.stream_mut().consume(|tok| match tok {
        TokenValue::Keyword(kw) => Some(*kw),
        _ => None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_col_label() {
        let source = "sequence xxyyzz character";
        let mut ctx = ParserContext::from(source);

        assert_eq!(Ok("sequence".into()), col_label(&mut ctx));
        assert_eq!(Ok("xxyyzz".into()), col_label(&mut ctx));
        assert_eq!(Ok("character".into()), col_label(&mut ctx));
    }
}

use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenValue;
