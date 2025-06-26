pub(super) fn publication(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        PUBLICATION ColId
    */

    let (_, name) = seq!(stream => Publication, col_id)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_publication() {
        test_parser!(
            source = "publication foo",
            parser = publication,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Publication;
