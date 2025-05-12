pub(super) fn extension() -> impl Combinator<Output = Str> {
    
    /*
        EXTENSION ColId
    */

    Extension
        .and_right(col_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_extension() {
        test_parser!(
            source = "extension foo",
            parser = extension(),
            expected = "foo".into()
        )
    }
}

use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
use postgres_parser_lexer::Keyword::Extension;
