/// Alias: `ReassignOwnedStmt`
pub(super) fn reassign_owned_stmt(ctx: &mut ParserContext) -> scan::Result<ReassignOwnedStmt> {

    /*
        REASSIGN OWNED BY role_list TO RoleSpec
    */

    let (.., roles, _, new_owner) = seq!(Reassign, OwnedKw, By, role_list, To, role_spec)
        .parse(ctx)?;

    Ok(ReassignOwnedStmt::new(roles, new_owner))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use pg_sink_ast::RoleSpec;

    #[test]
    fn test_reassign_owner_stmt() {
        test_parser!(
            source = "reassign owned by public, test_role to target_role",
            parser = reassign_owned_stmt,
            expected = ReassignOwnedStmt::new(
                vec![RoleSpec::Public, RoleSpec::Name("test_role".into())],
                RoleSpec::Name("target_role".into())
            )
        )
    }
}

use pg_ast::ReassignOwnedStmt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::OwnedKw;
use pg_lexer::Keyword::Reassign;
use pg_lexer::Keyword::To;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
use pg_sink_combinators::role_list;
use pg_sink_combinators::role_spec;
