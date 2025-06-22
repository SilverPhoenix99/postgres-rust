/// Aliases:
/// * `ColLabel`
/// * `attr_name`
pub(crate) fn col_label(stream: &mut TokenStream) -> Result<Str> {

    let parser = choice!(
        identifier,
        any_keyword()
    );

    parser.parse(stream)
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
use crate::combinators::foundation::choice;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::Str;
