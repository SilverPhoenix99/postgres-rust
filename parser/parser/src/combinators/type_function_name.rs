pub(super) fn type_function_name(stream: &mut TokenStream) -> Result<Str> {
    choice!(parsed stream =>
        identifier.map(Str::from),
        Unreserved.map(Str::from),
        TypeFuncName.map(Str::from),
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

        assert_eq!(Ok("before".into()), type_function_name(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), type_function_name(&mut stream));
        assert_eq!(Ok("collation".into()), type_function_name(&mut stream));
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
