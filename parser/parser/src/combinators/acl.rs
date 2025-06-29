pub(super) fn grantee_list(stream: &mut TokenStream) -> scan::Result<Vec<RoleSpec>> {

    /*
        grantee ( ',' grantee )*
    */

    many_sep(Comma, grantee).parse(stream)
}

fn grantee(stream: &mut TokenStream) -> scan::Result<RoleSpec> {

    /*
        ( GROUP )? role_spec
    */

    let (_, role) = (Group.optional(), role_spec)
        .parse(stream)?;

    Ok(role)
}

/// Alias: `opt_grant_grant_option`
pub(super) fn opt_grant_option(stream: &mut TokenStream<'_>) -> scan::Result<bool> {

    /*
        ( WITH GRANT OPTION )?
    */

    let grant = (With, Grant, OptionKw)
        .parse(stream)
        .optional()?
        .is_some();

    Ok(grant)
}

pub(super) fn opt_drop_behavior(stream: &mut TokenStream<'_>) -> scan::Result<DropBehavior> {

    /*
        ( CASCADE | RESTRICT )?
    */

    let behaviour = or((
        Cascade.map(|_| DropBehavior::Cascade),
        Restrict.map(|_| DropBehavior::Restrict)
    )).parse(stream);

    let behaviour = behaviour.optional()?
        .unwrap_or_default();

    Ok(behaviour)
}

pub(super) fn opt_granted_by(stream: &mut TokenStream<'_>) -> scan::Result<RoleSpec> {

    /*
        GRANTED BY role_spec
    */

    let (.., role) = (Granted, By, role_spec)
        .parse(stream)?;

    Ok(role)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::{test_parser, DEFAULT_CONFIG};

    #[test]
    fn test_grantee_list() {
        test_parser!(
            source = "group session_user, current_role",
            parser = grantee_list,
            expected = vec![
                RoleSpec::SessionUser,
                RoleSpec::CurrentRole
            ]
        )
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
        assert_eq!(Ok(true), opt_grant_option(&mut stream));
        assert_eq!(Ok(false), opt_grant_option(&mut stream));
    }

    #[test]
    fn test_opt_drop_behavior() {
        let mut stream = TokenStream::new("restrict cascade", DEFAULT_CONFIG);
        assert_eq!(Ok(DropBehavior::Restrict), opt_drop_behavior(&mut stream));
        assert_eq!(Ok(DropBehavior::Cascade), opt_drop_behavior(&mut stream));
        assert_eq!(Ok(DropBehavior::Restrict), opt_drop_behavior(&mut stream));
    }

    #[test]
    fn test_opt_granted_by() {
        test_parser!(
            source = "granted by public",
            parser = opt_granted_by,
            expected = RoleSpec::Public
        )
    }
}

use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_spec;
use crate::result::Optional;
use crate::scan;
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
