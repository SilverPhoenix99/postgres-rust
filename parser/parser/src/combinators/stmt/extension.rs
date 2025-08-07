pub(super) fn extension(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        EXTENSION ColId
    */

    let (_, name) = seq!(Extension, col_id).parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_extension() {
        test_parser!(
            source = "extension foo",
            parser = extension,
            expected = "foo"
        )
    }
}

use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Extension;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_combinators::col_id;
