fn locked_rels_list(ctx: &mut ParserContext) -> scan::Result<Vec<RelationName>> {

    /*
        OF qualified_name_list
    */

    let (_, rels) = seq!(Of, qualified_name_list).parse(ctx)?;
    Ok(rels)
}

/// Alias: `opt_nowait_or_skip`
fn nowait_or_skip(ctx: &mut ParserContext) -> scan::Result<LockWaitPolicy> {

    /*
          NOWAIT
        | SKIP LOCKED
    */

    alt!(
        Nowait.map(|_| WaitError),
        seq!(Skip, Locked).map(|_| WaitSkip)
    ).parse(ctx)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test]
    fn test_locked_rels_list() {
        test_parser!(
            source = "of foo, bar",
            parser = locked_rels_list,
            expected = vec![
                RelationName::new("foo"),
                RelationName::new("bar"),
            ]
        )
    }

    #[test_case("nowait" => Ok(WaitError))]
    #[test_case("skip locked" => Ok(WaitSkip))]
    fn test_nowait_or_skip(source: &str) -> scan::Result<LockWaitPolicy> {
        test_parser!(source, nowait_or_skip)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::qualified_name_list;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::LockWaitPolicy;
use pg_ast::LockWaitPolicy::WaitError;
use pg_ast::LockWaitPolicy::WaitSkip;
use pg_ast::RelationName;
use pg_lexer::Keyword::Locked;
use pg_lexer::Keyword::Nowait;
use pg_lexer::Keyword::Of;
use pg_lexer::Keyword::Skip;
use pg_parser_core::scan;
