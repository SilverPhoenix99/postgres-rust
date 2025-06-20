/// Alias: `NonReservedWord`
pub(super) fn non_reserved_word() -> impl Combinator<Output = Str> {
    parser(|stream|
        choice!(stream,
            identifier(stream).map(Str::from),
            Unreserved.parse(stream).map(Str::from),
            ColumnName.parse(stream).map(Str::from),
            TypeFuncName.parse(stream).map(Str::from),
        )
    )
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

        assert_eq!(Ok("breadth".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("boolean".into()), non_reserved_word().parse(&mut stream));
        assert_eq!(Ok("authorization".into()), non_reserved_word().parse(&mut stream));
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use pg_basics::Str;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
