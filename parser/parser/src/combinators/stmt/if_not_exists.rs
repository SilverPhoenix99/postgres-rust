pub(super) fn if_not_exists(ctx: &mut ParserContext) -> scan::Result<Presence> {

    let _ = seq!(If, Not, Exists).parse(ctx)?;

    Ok(Presence::Ignore)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_if_not_exists() {
        test_parser!(
            source = "if not exists",
            parser = if_not_exists,
            expected = Presence::Ignore
        )
    }
}

use pg_ast::Presence;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Exists;
use pg_lexer::Keyword::If;
use pg_lexer::Keyword::Not;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
