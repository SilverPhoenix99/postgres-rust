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
use crate::combinators::foundation::any_keyword;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::Combinator;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_parser_core::scan;
