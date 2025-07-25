/// Alias: `NonReservedWord`
pub(super) fn non_reserved_word(stream: &mut TokenStream) -> scan::Result<Str> {
    or((
        identifier.map(Str::from),
        Unreserved.map(Str::from),
        ColumnName.map(Str::from),
        TypeFuncName.map(Str::from),
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_non_reserved_word() {
        let source = "breadth xxyyzz boolean authorization";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("breadth".into()), non_reserved_word(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), non_reserved_word(&mut stream));
        assert_eq!(Ok("boolean".into()), non_reserved_word(&mut stream));
        assert_eq!(Ok("authorization".into()), non_reserved_word(&mut stream));
    }
}

use crate::combinators::foundation::identifier;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
