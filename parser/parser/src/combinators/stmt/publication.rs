pub(super) fn publication(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        PUBLICATION ColId
    */

    let (_, name) = seq!(Publication, col_id)
        .parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_publication() {
        test_parser!(
            source = "publication foo",
            parser = publication,
            expected = "foo"
        )
    }
}

use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_identifier_combinators::col_id;
use pg_lexer::Keyword::Publication;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
