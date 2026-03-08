pub(super) fn if_exists(ctx: &mut ParserContext) -> scan::Result<Presence> {

    /*
        IF EXISTS
    */

    let _ = seq!(If, Exists).parse(ctx)?;

    Ok(Presence::Ignore)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_if_exists() {
        test_parser!(
            source = "if exists",
            parser = if_exists,
            expected = Presence::Ignore
        )
    }
}

use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::Presence;
use pg_lexer::Keyword::Exists;
use pg_lexer::Keyword::If;
use pg_parser_core::scan;
