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
pub(super) fn with_grant_option(stream: &mut TokenStream<'_>) -> scan::Result<GrantOption> {

    /*
        WITH GRANT OPTION
    */

    let _ = (With, Grant, OptionKw)
        .parse(stream)?;

    Ok(GrantOption::WithGrant)
}

/// Alias: `opt_drop_behavior`
pub(super) fn drop_behavior(stream: &mut TokenStream<'_>) -> scan::Result<DropBehavior> {

    /*
        CASCADE | RESTRICT
    */

    or((
        Cascade.map(|_| DropBehavior::Cascade),
        Restrict.map(|_| DropBehavior::Restrict)
    )).parse(stream)
}

/// Alias: `opt_granted_by`
pub(super) fn granted_by(stream: &mut TokenStream<'_>) -> scan::Result<RoleSpec> {

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
    fn test_with_grant_option() {
        test_parser!(
            source = "with grant option",
            parser = with_grant_option,
            expected = GrantOption::WithGrant
        )
    }

    #[test]
    fn test_drop_behavior() {
        let mut stream = TokenStream::new("restrict cascade", DEFAULT_CONFIG);
        assert_eq!(Ok(DropBehavior::Restrict), drop_behavior(&mut stream));
        assert_eq!(Ok(DropBehavior::Cascade), drop_behavior(&mut stream));
    }

    #[test]
    fn test_granted_by() {
        test_parser!(
            source = "granted by public",
            parser = granted_by,
            expected = RoleSpec::Public
        )
    }
}

use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_spec;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::DropBehavior;
use pg_ast::GrantOption;
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
