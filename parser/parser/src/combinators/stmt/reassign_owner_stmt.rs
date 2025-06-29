/// Alias: `ReassignOwnedStmt`
pub(super) fn reassign_owned_stmt(stream: &mut TokenStream) -> scan::Result<ReassignOwnedStmt> {

    /*
        REASSIGN OWNED BY role_list TO RoleSpec
    */

    let (.., roles, _, new_owner) = (Reassign, OwnedKw, By, role_list, To, role_spec)
        .parse(stream)?;

    Ok(ReassignOwnedStmt::new(roles, new_owner))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::RoleSpec;

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

use crate::combinators::foundation::Combinator;
use crate::combinators::role_list;
use crate::combinators::role_spec;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ReassignOwnedStmt;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::OwnedKw;
use pg_lexer::Keyword::Reassign;
use pg_lexer::Keyword::To;
