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
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_type_function_name() {
        let source = "before xxyyzz collation";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok("before".into()), type_function_name().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), type_function_name().parse(&mut stream));
        assert_eq!(Ok("collation".into()), type_function_name().parse(&mut stream));
    }
}

use crate::lexer::KeywordCategory::TypeFuncName;
use crate::lexer::KeywordCategory::Unreserved;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::foundation::{identifier, match_first};
use postgres_basics::Str;
