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
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_col_id() {
        let source = "cascaded xxyyzz coalesce";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("cascaded".into()), col_id().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), col_id().parse(&mut stream));
        assert_eq!(Ok("coalesce".into()), col_id().parse(&mut stream));
    }
}

use crate::lexer::KeywordCategory::ColumnName;
use crate::lexer::KeywordCategory::Unreserved;
use crate::parser::combinators::foundation::identifier;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
