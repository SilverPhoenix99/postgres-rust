/// Alias: `BareColLabel`
pub fn bare_col_label(stream: &mut TokenStream<'_>) -> scan::Result<Str> {
    alt!(
        identifier.map(From::from),
        keyword_if(|kw| kw.details().bare()).map(From::from)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bare_col_label() {
        let source = "sequence xxyyzz";
        let mut stream = TokenStream::from(source);

        assert_eq!(Ok("sequence".into()), bare_col_label(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), bare_col_label(&mut stream));
    }
}

use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::keyword_if;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
