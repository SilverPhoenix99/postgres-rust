/// Aliases:
/// * `ColId`
/// * `name`
pub(super) fn col_id() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        Unreserved.map(From::from),
        ColumnName.map(From::from),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_col_id() {
        let source = "cascaded xxyyzz coalesce";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("cascaded".into()), col_id().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), col_id().parse(&mut stream));
        assert_eq!(Ok("coalesce".into()), col_id().parse(&mut stream));
    }
}

use crate::combinators::foundation::identifier;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
use postgres_parser_lexer::KeywordCategory::ColumnName;
use postgres_parser_lexer::KeywordCategory::Unreserved;
