/// Aliases:
/// * `ColLabel`
/// * `attr_name`
pub(in crate::combinators) fn col_label(stream: &mut TokenStream) -> scan::Result<Str> {

    or((
        identifier.map(From::from),
        any_keyword.map(From::from)
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::stream;

    #[test]
    fn test_col_label() {
        let source = "sequence xxyyzz character";
        let mut stream = stream(source);

        assert_eq!(Ok("sequence".into()), col_label(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), col_label(&mut stream));
        assert_eq!(Ok("character".into()), col_label(&mut stream));
    }
}

use crate::combinators::foundation::any_keyword;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
