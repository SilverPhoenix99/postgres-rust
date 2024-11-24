/// Alias: `ReassignOwnedStmt`
pub(in crate::parser) fn reassign_owned_stmt() -> impl Combinator<Output = ReassignOwnedStmt> {

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

use crate::lexer::Keyword::By;
use crate::lexer::Keyword::OwnedKw;
use crate::lexer::Keyword::Reassign;
use crate::lexer::Keyword::To;
use crate::parser::ast_node::ReassignOwnedStmt;
use crate::parser::combinators::sequence;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::role_parsers::role_list;
use crate::parser::role_parsers::role_spec;
