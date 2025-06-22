pub(super) fn extension() -> impl Combinator<Output = Str> {

    /*
        EXTENSION ColId
    */

    Extension
        .and_right(col_id)
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
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::Combinator;
use pg_basics::Str;
use pg_lexer::Keyword::Extension;
