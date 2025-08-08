/// Alias: `opt_drop_behavior`
pub fn drop_behavior(stream: &mut TokenStream<'_>) -> scan::Result<DropBehavior> {

    /*
        CASCADE | RESTRICT
    */

    alt!(
        Cascade.map(|_| DropBehavior::Cascade),
        Restrict.map(|_| DropBehavior::Restrict)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("restrict" => Ok(DropBehavior::Restrict))]
    #[test_case("cascade" => Ok(DropBehavior::Cascade))]
    fn test_drop_behavior(source: &str) -> scan::Result<DropBehavior> {
        test_parser!(source, drop_behavior)
    }
}

use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Cascade;
use pg_lexer::Keyword::Restrict;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_ast::DropBehavior;
