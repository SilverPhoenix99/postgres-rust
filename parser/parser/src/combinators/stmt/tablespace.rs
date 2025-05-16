pub(super) fn tablespace() -> impl Combinator<Output = Str> {
    
    /*
        TABLESPACE ColId
    */

    Tablespace
        .and_right(col_id())
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
            expected = "foo".into()
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
use postgres_parser_lexer::Keyword::Tablespace;
