pub(super) fn if_not_exists(stream: &mut TokenStream) -> scan::Result<Presence> {

    let _ = (If, Not, Exists).parse(stream)?;

    Ok(Presence::Ignore)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_if_not_exists() {
        test_parser!(
            source = "if not exists",
            parser = if_not_exists,
            expected = Presence::Ignore
        )
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::Presence;
use pg_lexer::Keyword::Exists;
use pg_lexer::Keyword::If;
use pg_lexer::Keyword::Not;
