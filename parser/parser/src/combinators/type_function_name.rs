pub(super) fn type_function_name() -> impl Combinator<Output = Str> {
    choice!(
        identifier,
        Unreserved,
        TypeFuncName,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_type_function_name() {
        let source = "before xxyyzz collation";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("before".into()), type_function_name().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), type_function_name().parse(&mut stream));
        assert_eq!(Ok("collation".into()), type_function_name().parse(&mut stream));
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::Combinator;
use pg_basics::Str;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
