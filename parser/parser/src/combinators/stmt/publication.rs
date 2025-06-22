pub(super) fn publication() -> impl Combinator<Output = Str> {

    /*
        PUBLICATION ColId
    */

    Publication
        .and_right(parser(col_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_publication() {
        test_parser!(
            source = "publication foo",
            parser = publication(),
            expected = "foo".into()
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::{parser, Combinator};
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::Str;
use pg_lexer::Keyword::Publication;
