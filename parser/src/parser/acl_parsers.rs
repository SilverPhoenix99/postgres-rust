/// Post-condition: Vec is **Not** empty
pub(super) fn grantee_list() -> impl Combinator<Output = Vec<RoleSpec>> {

    /*
        grantee ( ',' grantee )*
    */
    
    many_sep(operator(Comma), grantee())
}

fn grantee() -> impl Combinator<Output = RoleSpec> {

    /*
        ( GROUP )? role_spec
    */

    keyword(Group).maybe_match()
        .and_right(role_spec())
}

/// Alias: `opt_grant_grant_option`
pub(super) fn opt_grant_option() -> impl Combinator<Output = bool> {

    /*
        ( WITH GRANT OPTION )?
    */

    keyword(With)
        .and(keyword(Grant))
        .and(keyword(OptionKw))
        .optional()
        .map(|grant| grant.is_some())
}

pub(super) fn opt_drop_behavior() -> impl Combinator<Output = DropBehavior> {

    /*
        ( CASCADE | RESTRICT )?
    */

    keyword(Cascade).map(|_| DropBehavior::Cascade)
        .or(keyword(Restrict).map(|_| DropBehavior::Restrict))
        .optional()
        .map(Option::unwrap_or_default)
}

pub(super) fn opt_granted_by() -> impl Combinator<Output = RoleSpec> {

    /*
        GRANTED BY role_spec
    */

    keyword(Granted)
        .and(keyword(By))
        .and_right(role_spec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_grantee_list() {
        let source = "group session_user, current_role";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = vec![
            RoleSpec::SessionUser,
            RoleSpec::CurrentRole
        ];

        assert_eq!(Ok(expected), grantee_list().parse(&mut stream));
    }

    #[test]
    fn test_grantee() {
        let source = "current_user group public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(RoleSpec::CurrentUser), grantee().parse(&mut stream));
        assert_eq!(Ok(RoleSpec::Public), grantee().parse(&mut stream));
    }

    #[test]
    fn test_opt_grant_option() {
        let mut stream = TokenStream::new("with grant option", DEFAULT_CONFIG);
        let parser = opt_grant_option();
        assert_eq!(Ok(true), parser.parse(&mut stream));
        assert_eq!(Ok(false), parser.parse(&mut stream));
    }

    #[test]
    fn test_opt_drop_behavior() {
        let mut stream = TokenStream::new("restrict cascade", DEFAULT_CONFIG);
        assert_eq!(Ok(DropBehavior::Restrict), opt_drop_behavior().parse(&mut stream));
        assert_eq!(Ok(DropBehavior::Cascade), opt_drop_behavior().parse(&mut stream));
        assert_eq!(Ok(DropBehavior::Restrict), opt_drop_behavior().parse(&mut stream));
    }

    #[test]
    fn test_opt_granted_by() {
        let mut stream = TokenStream::new("granted by public", DEFAULT_CONFIG);
        assert_eq!(Ok(RoleSpec::Public), opt_granted_by().parse(&mut stream));
    }
}

use crate::lexer::Keyword::By;
use crate::lexer::Keyword::Cascade;
use crate::lexer::Keyword::Grant;
use crate::lexer::Keyword::Granted;
use crate::lexer::Keyword::Group;
use crate::lexer::Keyword::OptionKw;
use crate::lexer::Keyword::Restrict;
use crate::lexer::Keyword::With;
use crate::lexer::OperatorKind::Comma;
use crate::parser::ast_node::DropBehavior;
use crate::parser::ast_node::RoleSpec;
use crate::parser::combinators::{keyword, many_sep, operator, Combinator, CombinatorHelpers};
use crate::parser::role_parsers::role_spec;
