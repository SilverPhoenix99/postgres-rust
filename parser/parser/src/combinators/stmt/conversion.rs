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
    use crate::tests::test_parser;

    #[test]
    fn test_conversion() {
        test_parser!(
            source = "conversion foo",
            parser = conversion(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_basics::QualifiedName;
use postgres_parser_lexer::Keyword::Conversion;
