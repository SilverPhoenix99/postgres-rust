pub(super) fn extension() -> impl Combinator<Output = Str> {

    /*
        EXTENSION ColId
    */

    Extension
        .and_right(parser(col_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_extension() {
        test_parser!(
            source = "extension foo",
            parser = extension(),
            expected = "foo".into()
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::{parser, Combinator};
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::Str;
use pg_lexer::Keyword::Extension;
