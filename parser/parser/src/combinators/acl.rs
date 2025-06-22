pub(super) fn grantee_list() -> impl Combinator<Output = Vec<RoleSpec>> {

    /*
        grantee ( ',' grantee )*
    */

    many!(sep = Comma, grantee)
}

fn grantee(stream: &mut TokenStream) -> Result<RoleSpec> {

    /*
        ( GROUP )? role_spec
    */

    let parser = seq!(Group.maybe_match(), role_spec)
        .right();

    parser.parse(stream)
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
        .and_right(role_spec)
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
        assert_eq!(Ok(RoleSpec::CurrentUser), grantee(&mut stream));
        assert_eq!(Ok(RoleSpec::Public), grantee(&mut stream));
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

use crate::combinators::foundation::{many, CombinatorHelpers};
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_spec;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::DropBehavior;
use pg_ast::RoleSpec;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Cascade;
use pg_lexer::Keyword::Grant;
use pg_lexer::Keyword::Granted;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::OptionKw;
use pg_lexer::Keyword::Restrict;
use pg_lexer::Keyword::With;
use pg_lexer::OperatorKind::Comma;
