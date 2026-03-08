pub(in crate::combinators::stmt) fn publication(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        PUBLICATION ColId
    */

    let (_, name) = seq!(Publication, col_id)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

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
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Publication;
use pg_parser_core::scan;
