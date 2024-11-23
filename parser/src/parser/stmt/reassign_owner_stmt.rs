/// Alias: `ReassignOwnedStmt`
pub(in crate::parser) fn reassign_owned_stmt() -> impl Combinator<Output = ReassignOwnedStmt> {

    /*
        REASSIGN OWNED BY role_list TO RoleSpec
    */

    sequence!(
        keyword(Reassign)
            .and(keyword(OwnedKw))
            .and(keyword(By))
            .skip(),
        role_list(),
        keyword(To).skip(),
        role_spec()
    ).map(|(|_, roles, _, new_owner)| {
        ReassignOwnedStmt::new(roles, new_owner)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

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

use crate::lexer::Keyword::{By, OwnedKw, Reassign, To};
use crate::parser::ast_node::ReassignOwnedStmt;
use crate::parser::combinators::{keyword, sequence, Combinator, CombinatorHelpers};
use crate::parser::role_parsers::{role_list, role_spec};
