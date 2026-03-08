/// Alias: `opt_drop_behavior`
pub(super) fn drop_behavior(ctx: &mut ParserContext) -> scan::Result<DropBehavior> {

    /*
        CASCADE | RESTRICT
    */

    alt!(
        Cascade.map(|_| DropBehavior::Cascade),
        Restrict.map(|_| DropBehavior::Restrict)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("restrict" => Ok(DropBehavior::Restrict))]
    #[test_case("cascade" => Ok(DropBehavior::Cascade))]
    fn test_drop_behavior(source: &str) -> scan::Result<DropBehavior> {
        test_parser!(source, drop_behavior)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::ParserContext;
use pg_ast::DropBehavior;
use pg_lexer::Keyword::Cascade;
use pg_lexer::Keyword::Restrict;
use pg_parser_core::scan;
