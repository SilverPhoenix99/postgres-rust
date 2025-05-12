/// Alias: `NonReservedWord`
pub(super) fn non_reserved_word() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        Unreserved.map(From::from),
        ColumnName.map(From::from),
        TypeFuncName.map(From::from),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_non_reserved_word() {
        let source = "breadth xxyyzz boolean authorization";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("breadth".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("boolean".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("authorization".into()), non_reserved_word().parse(&mut stream));
    }
}

use crate::parser::combinators::foundation::identifier;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
use postgres_parser_lexer::KeywordCategory::ColumnName;
use postgres_parser_lexer::KeywordCategory::TypeFuncName;
use postgres_parser_lexer::KeywordCategory::Unreserved;
