pub(super) fn if_exists(stream: &mut TokenStream) -> scan::Result<Presence> {

    /*
        IF EXISTS
    */

    let _ = seq!(If, Exists).parse(stream)?;

    Ok(Presence::Ignore)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_if_exists() {
        test_parser!(
            source = "if exists",
            parser = if_exists,
            expected = Presence::Ignore
        )
    }
}

use crate::combinators::foundation::seq;
use pg_ast::Presence;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Exists;
use pg_lexer::Keyword::If;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
