pub(super) fn tablespace() -> impl Combinator<Output = Str> {

    /*
        TABLESPACE ColId
    */

    Tablespace
        .and_right(col_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_tablespace() {
        test_parser!(
            source = "tablespace foo",
            parser = tablespace(),
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::Combinator;
use pg_basics::Str;
use pg_lexer::Keyword::Tablespace;
