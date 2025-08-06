/// Alias: `BareColLabel`
pub(super) fn bare_col_label(stream: &mut TokenStream<'_>) -> scan::Result<Str> {
    alt!(
        identifier.map(From::from),
        keyword_if(|kw| kw.details().bare()).map(From::from)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;

    #[test]
    fn test_bare_col_label() {
        let source = "sequence xxyyzz";
        let mut stream = TokenStream::from(source);

        assert_eq!(Ok("sequence".into()), bare_col_label(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), bare_col_label(&mut stream));
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::keyword_if;
use crate::combinators::foundation::Combinator;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_parser_core::scan;
