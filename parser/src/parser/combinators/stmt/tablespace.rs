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
    use crate::parser::tests::test_parser;

    #[test]
    fn test_tablespace() {
        test_parser!(
            source = "tablespace foo",
            parser = tablespace(),
            expected = "foo".into()
        )
    }
}

use crate::lexer::Keyword::Tablespace;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
