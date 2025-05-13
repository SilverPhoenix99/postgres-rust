pub(super) fn conversion() -> impl Combinator<Output = QualifiedName> {

    /*
        CONVERSION any_name
    */

    Conversion
        .and_right(any_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_conversion() {
        test_parser!(
            source = "conversion foo",
            parser = conversion(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::QualifiedName;
use postgres_parser_lexer::Keyword::Conversion;
