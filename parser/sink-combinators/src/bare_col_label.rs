/// Alias: `BareColLabel`
pub fn bare_col_label(ctx: &mut ParserContext) -> scan::Result<Str> {
    alt!(
        identifier.map(From::from),
        keyword_if(|kw| kw.details().bare()).map(From::from)
    ).parse(ctx)
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
}

use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::keyword_if;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
