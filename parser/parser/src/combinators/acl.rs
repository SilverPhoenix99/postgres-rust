/// Post-condition: Vec is **Not** empty
pub(super) fn grantee_list() -> impl Combinator<Output = Vec<RoleSpec>> {

    /*
        grantee ( ',' grantee )*
    */

    many_sep(Comma, grantee())
}

fn grantee() -> impl Combinator<Output = RoleSpec> {

    /*
        ( GROUP )? role_spec
    */

    Group.maybe_match()
        .and_right(role_spec())
}

/// Alias: `opt_grant_grant_option`
pub(super) fn opt_grant_option() -> impl Combinator<Output = bool> {

    /*
        ( WITH GRANT OPTION )?
    */

    With.and(Grant).and(OptionKw)
        .optional()
        .map(|grant| grant.is_some())
}

pub(super) fn opt_drop_behavior() -> impl Combinator<Output = DropBehavior> {

    /*
        ( CASCADE | RESTRICT )?
    */

    Cascade.map(|_| DropBehavior::Cascade)
        .or(Restrict.map(|_| DropBehavior::Restrict))
        .optional()
        .map(Option::unwrap_or_default)
}

pub(super) fn opt_granted_by() -> impl Combinator<Output = RoleSpec> {

    /*
        GRANTED BY role_spec
    */

    Granted.and(By)
        .and_right(role_spec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

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

use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::role_spec;
use postgres_parser_ast::DropBehavior;
use postgres_parser_ast::RoleSpec;
use postgres_parser_lexer::Keyword::By;
use postgres_parser_lexer::Keyword::Cascade;
use postgres_parser_lexer::Keyword::Grant;
use postgres_parser_lexer::Keyword::Granted;
use postgres_parser_lexer::Keyword::Group;
use postgres_parser_lexer::Keyword::OptionKw;
use postgres_parser_lexer::Keyword::Restrict;
use postgres_parser_lexer::Keyword::With;
use postgres_parser_lexer::OperatorKind::Comma;
