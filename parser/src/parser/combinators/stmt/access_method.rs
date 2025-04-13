pub(super) fn access_method() -> impl Combinator<Output = Str> {

    /*
        ACCESS METHOD ColId
    */

    and(Access, Method)
        .and_right(col_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_access_method() {
        test_parser!(
            source = "access method foo",
            parser = access_method(),
            expected = "foo".into()
        )
    }
}

use crate::lexer::Keyword::Access;
use crate::lexer::Keyword::Method;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
