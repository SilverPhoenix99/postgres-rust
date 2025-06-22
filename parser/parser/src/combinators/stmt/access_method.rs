pub(super) fn access_method() -> impl Combinator<Output = Str> {

    /*
        ACCESS METHOD ColId
    */

    and(Access, Method)
        .and_right(col_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_access_method() {
        test_parser!(
            source = "access method foo",
            parser = access_method(),
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::and;
use crate::combinators::foundation::Combinator;
use pg_basics::Str;
use pg_lexer::Keyword::Access;
use pg_lexer::Keyword::Method;
