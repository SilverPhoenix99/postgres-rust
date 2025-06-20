pub(super) fn view() -> impl Combinator<Output = QualifiedName> {

    /*
        VIEW any_name
    */

    View
        .and_right(parser(any_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_view() {
        test_parser!(
            source = "view foo",
            parser = view(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::View;
