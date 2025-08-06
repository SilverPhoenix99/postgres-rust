/// Aliases:
/// * `ColLabel`
/// * `attr_name`
pub(in crate::combinators) fn col_label(stream: &mut TokenStream) -> scan::Result<Str> {

    alt!(
        identifier.map(From::from),
        any_keyword.map(From::from)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_col_label() {
        let source = "sequence xxyyzz character";
        let mut stream = TokenStream::from(source);

        assert_eq!(Ok("sequence".into()), col_label(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), col_label(&mut stream));
        assert_eq!(Ok("character".into()), col_label(&mut stream));
    }
}

use crate::combinators::foundation::alt;
use pg_basics::Str;
use pg_combinators::any_keyword;
use pg_combinators::identifier;
use pg_combinators::Combinator;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
