pub(super) fn database() -> impl Combinator<Output = Str> {

    /*
        DATABASE ColId
    */

    Database
        .and_right(col_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_database() {
        test_parser!(
            source = "database foo",
            parser = database(),
            expected = "foo".into()
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
use postgres_parser_lexer::Keyword::Database;
