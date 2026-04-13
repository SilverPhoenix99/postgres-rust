fn for_locking_items(ctx: &mut ParserContext) -> scan::Result<Vec<LockingClause>> {

    /*
        ( for_locking_item )+
    */

    many!(for_locking_item).parse(ctx)
}

fn for_locking_item(ctx: &mut ParserContext) -> scan::Result<LockingClause> {

    /*
        for_locking_strength locked_rels_list ( nowait_or_skip )?
    */

    let (strength, rels, wait_policy) = seq!(
        for_locking_strength,
        locked_rels_list,
        nowait_or_skip.optional()
    ).parse(ctx)?;

    let wait_policy = wait_policy.unwrap_or_default();
    let locking_clause = LockingClause::new(rels, strength, wait_policy);
    Ok(locking_clause)
}

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
    #[allow(unused_imports)]
    use pg_ast::{
        LockClauseStrength::ForNoKeyUpdate,
        LockClauseStrength::ForUpdate,
        LockWaitPolicy::Block,
    };
    use test_case::test_case;

    #[test_case("for update of foo, bar nowait for no key update of qux" => matches Ok(_))]
    fn test_for_locking_items(source: &str) -> scan::Result<Vec<LockingClause>> {
        test_parser!(source, for_locking_items)
    }

    #[test_case("for update of foo, bar nowait" => Ok(
        LockingClause::new(
            vec![
                RelationName::new("foo"),
                RelationName::new("bar"),
            ],
            ForUpdate,
            WaitError
        )
    ))]
    #[test_case("for no key update of qux" => Ok(
        LockingClause::new(
            vec![
                RelationName::new("qux"),
            ],
            ForNoKeyUpdate,
            Block
        )
    ))]
    fn test_for_locking_item(source: &str) -> scan::Result<LockingClause> {
        test_parser!(source, for_locking_item)
    }

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
use crate::combinators::stmt::for_locking_strength;
use crate::context::ParserContext;
use crate::many;
use crate::seq;
use pg_ast::LockWaitPolicy;
use pg_ast::LockWaitPolicy::WaitError;
use pg_ast::LockWaitPolicy::WaitSkip;
use pg_ast::LockingClause;
use pg_ast::RelationName;
use pg_lexer::Keyword::Locked;
use pg_lexer::Keyword::Nowait;
use pg_lexer::Keyword::Of;
use pg_lexer::Keyword::Skip;
use pg_parser_core::scan;
