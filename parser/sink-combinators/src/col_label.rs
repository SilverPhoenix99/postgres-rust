/// Aliases:
/// * `ColLabel`
/// * `attr_name`
pub fn col_label(ctx: &mut ParserContext) -> scan::Result<Str> {

    alt!(
        identifier.map(From::from),
        any_keyword.map(From::from)
    ).parse(ctx)
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
use pg_combinators::any_keyword;
use pg_combinators::identifier;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
