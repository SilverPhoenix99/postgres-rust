pub(super) fn language() -> impl Combinator<Output = Str> {

    /*
        opt_procedural LANGUAGE name
    */

    or(
        Language.skip(),
        and(Procedural, Language).skip()
    )
        .and_right(col_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_language() {
        test_parser!(
            source = "language foo",
            parser = language(),
            expected = "foo"
        );
    }

    #[test]
    fn test_procedural_language() {
    test_parser!(
            source = "procedural language foo",
            parser = language(),
            expected = "foo"
        );
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::and;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use pg_basics::Str;
use pg_lexer::Keyword::Language;
use pg_lexer::Keyword::Procedural;
