pub(super) fn publication() -> impl Combinator<Output = Str> {

    /*
        PUBLICATION ColId
    */

    Publication
        .and_right(col_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_publication() {
        test_parser!(
            source = "publication foo",
            parser = publication(),
            expected = "foo".into()
        )
    }
}

use crate::lexer::Keyword::Publication;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
