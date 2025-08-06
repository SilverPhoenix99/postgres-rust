/// Aliases:
/// * `ColId`
/// * `name`
/// * `opt_single_name`
pub fn col_id(stream: &mut TokenStream) -> scan::Result<Str> {

    alt!(
        identifier.map(From::from),
        Unreserved.map(From::from),
        ColumnName.map(From::from)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_col_id() {
        let source = "cascaded xxyyzz coalesce";
        let mut stream = TokenStream::from(source);

        assert_eq!(Ok("cascaded".into()), col_id(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), col_id(&mut stream));
        assert_eq!(Ok("coalesce".into()), col_id(&mut stream));
    }
}

use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::Combinator;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
