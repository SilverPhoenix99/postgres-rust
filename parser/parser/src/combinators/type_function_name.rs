pub(super) fn type_function_name() -> impl Combinator<Output = Str> {
    match_first!(
        identifier().map(From::from),
        Unreserved.map(From::from),
        TypeFuncName.map(From::from),
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

use crate::combinators::foundation::identifier;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
use postgres_parser_lexer::KeywordCategory::TypeFuncName;
use postgres_parser_lexer::KeywordCategory::Unreserved;
