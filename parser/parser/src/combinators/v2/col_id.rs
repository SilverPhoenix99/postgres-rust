/// Aliases:
/// * `ColId`
/// * `name`
pub(in crate::combinators) fn col_id(stream: &mut TokenStream) -> crate::scan::Result<Str> {
    choice!(stream,
        identifier(stream),
        Unreserved.parse(stream),
        ColumnName.parse(stream)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::stream;

    #[test]
    fn test_col_id() {
        let source = "cascaded xxyyzz coalesce";
        let mut stream = stream(source);

        assert_eq!(Ok("cascaded".into()), col_id(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), col_id(&mut stream));
        assert_eq!(Ok("coalesce".into()), col_id(&mut stream));
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::Combinator;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::Unreserved;
