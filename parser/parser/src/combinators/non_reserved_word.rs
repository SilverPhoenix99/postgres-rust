/// Alias: `NonReservedWord`
pub(super) fn non_reserved_word(stream: &mut TokenStream) -> scan::Result<Str> {
    alt!(
        identifier.map(Str::from),
        Unreserved.map(Str::from),
        ColumnName.map(Str::from),
        TypeFuncName.map(Str::from),
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_reserved_word() {
        let source = "breadth xxyyzz boolean authorization";
        let mut stream = TokenStream::from(source);

        assert_eq!(Ok("breadth".into()), non_reserved_word(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), non_reserved_word(&mut stream));
        assert_eq!(Ok("boolean".into()), non_reserved_word(&mut stream));
        assert_eq!(Ok("authorization".into()), non_reserved_word(&mut stream));
    }
}

use crate::combinators::foundation::alt;
use pg_basics::Str;
use pg_combinators::identifier;
use pg_combinators::Combinator;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
