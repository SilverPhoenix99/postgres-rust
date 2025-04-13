pub(super) fn statistics() -> impl Combinator<Output = QualifiedName> {

    /*
        STATISTICS any_name
    */

    Statistics
        .and_right(any_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_statistics() {
        test_parser!(
            source = "statistics foo",
            parser = statistics(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::lexer::Keyword::Statistics;
use crate::parser::ast_node::QualifiedName;
use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
