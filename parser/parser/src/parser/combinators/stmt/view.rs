pub(super) fn view() -> impl Combinator<Output = QualifiedName> {

    /*
        VIEW any_name
    */

    View
        .and_right(any_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_view() {
        test_parser!(
            source = "view foo",
            parser = view(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::QualifiedName;
use postgres_parser_lexer::Keyword::View;
