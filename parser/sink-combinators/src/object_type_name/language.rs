pub fn language(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        ( PROCEDURAL )? LANGUAGE name
    */

    let (_, name) = seq!(
        seq!(Procedural.optional(), Language),
        col_id
    ).parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

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

use crate::col_id;
use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Language;
use pg_lexer::Keyword::Procedural;
use pg_parser_core::scan;
