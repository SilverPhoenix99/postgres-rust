pub(super) fn language(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        ( PROCEDURAL )? LANGUAGE name
    */

    let (_, name) = seq!(
        alt!(
            Language.skip(),
            seq!(Procedural, Language).skip()
        ),
        col_id
    ).parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_language() {
        test_parser!(
            source = "language foo",
            parser = language,
            expected = "foo"
        );
    }

    #[test]
    fn test_procedural_language() {
    test_parser!(
            source = "procedural language foo",
            parser = language,
            expected = "foo"
        );
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Language;
use pg_lexer::Keyword::Procedural;
