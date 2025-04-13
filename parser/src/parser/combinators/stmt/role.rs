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
    use crate::parser::tests::test_parser;

    #[test]
    fn test_role() {
        test_parser!(
            source = "role foo",
            parser = role(),
            expected = "foo".into()
        )
    }
}

use crate::lexer::Keyword::Role;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
