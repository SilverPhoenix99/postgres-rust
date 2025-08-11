/// Alias: `BareColLabel`
pub fn bare_col_label(ctx: &mut ParserContext) -> scan::Result<Str> {
    alt!(
        identifier.map(From::from),
        bare_keyword
    ).parse(ctx)
}

fn bare_keyword(ctx: &mut ParserContext) -> scan::Result<Str> {
    ctx.stream_mut().consume(|tok| match tok {
        TokenValue::Keyword(kw) if kw.details().bare() => Some((*kw).into()),
        _ => None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bare_col_label() {
        let source = "sequence xxyyzz";
        let mut ctx = ParserContext::from(source);

        assert_eq!(Ok("sequence".into()), bare_col_label(&mut ctx));
        assert_eq!(Ok("xxyyzz".into()), bare_col_label(&mut ctx));
    }

    #[test]
    fn test_bare_keyword() {
        let source = "sequence";
        let mut ctx = ParserContext::from(source);
        assert_eq!(Ok("sequence".into()), bare_keyword(&mut ctx));
    }
}

use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenValue;
