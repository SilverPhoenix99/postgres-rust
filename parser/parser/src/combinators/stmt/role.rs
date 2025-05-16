pub(super) fn role() -> impl Combinator<Output = Str> {

    /*
        ROLE name
    */

    Role
        .and_right(col_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_role() {
        test_parser!(
            source = "role foo",
            parser = role(),
            expected = "foo".into()
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::Str;
use pg_lexer::Keyword::Role;
