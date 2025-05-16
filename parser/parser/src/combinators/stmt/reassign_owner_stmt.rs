/// Alias: `ReassignOwnedStmt`
pub(super) fn reassign_owned_stmt() -> impl Combinator<Output = ReassignOwnedStmt> {

    /*
        REASSIGN OWNED BY role_list TO RoleSpec
    */

    sequence!(
        Reassign.and(OwnedKw).and(By).skip(),
        role_list(),
        To.skip(),
        role_spec()
    ).map(|(|_, roles, _, new_owner)| {
        ReassignOwnedStmt::new(roles, new_owner)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::RoleSpec;

    #[test]
    fn test_reassign_owner_stmt() {
        let mut stream = TokenStream::new("reassign owned by public, test_role to target_role", DEFAULT_CONFIG);

        let expected = ReassignOwnedStmt::new(
            vec![RoleSpec::Public, RoleSpec::Name("test_role".into())],
            RoleSpec::Name("target_role".into())
        );

        assert_eq!(Ok(expected), reassign_owned_stmt().parse(&mut stream));
    }
}

use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::role_list;
use crate::combinators::role_spec;
use pg_ast::ReassignOwnedStmt;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::OwnedKw;
use pg_lexer::Keyword::Reassign;
use pg_lexer::Keyword::To;
