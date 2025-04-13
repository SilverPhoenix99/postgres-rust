pub(super) fn language() -> impl Combinator<Output = Str> {

    /*
        opt_procedural LANGUAGE name
    */

    or(
        Language.skip(),
        and(Procedural, Language).skip()
    )
        .and_right(col_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_language() {
        test_parser!(
            source = "language foo",
            parser = language(),
            expected = "foo".into()
        );
    }

    #[test]
    fn test_procedural_language() {
    test_parser!(
            source = "procedural language foo",
            parser = language(),
            expected = "foo".into()
        );
    }
}

use crate::lexer::Keyword::Language;
use crate::lexer::Keyword::Procedural;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
