pub(super) fn grantee_list(ctx: &mut ParserContext) -> scan::Result<Vec<RoleSpec>> {

    /*
        grantee ( ',' grantee )*
    */

    many!(sep = Comma, grantee).parse(ctx)
}

fn grantee(ctx: &mut ParserContext) -> scan::Result<RoleSpec> {

    /*
        ( GROUP )? role_spec
    */

    let (_, role) = seq!(Group.optional(), role_spec)
        .parse(ctx)?;

    Ok(role)
}

/// Alias: `opt_grant_grant_option`
pub(super) fn with_grant_option(ctx: &mut ParserContext) -> scan::Result<GrantOption> {

    /*
        WITH GRANT OPTION
    */

    let _ = seq!(With, Grant, OptionKw)
        .parse(ctx)?;

    Ok(GrantOption::WithGrant)
}

/// Alias: `opt_granted_by`
pub(super) fn granted_by(ctx: &mut ParserContext) -> scan::Result<RoleSpec> {

    /*
        GRANTED BY role_spec
    */

    let (.., role) = seq!(Granted, By, role_spec)
        .parse(ctx)?;

    Ok(role)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

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
        let mut ctx = ParserContext::from(source);
        assert_eq!(Ok(RoleSpec::CurrentUser), grantee(&mut ctx));
        assert_eq!(Ok(RoleSpec::Public), grantee(&mut ctx));
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
    fn test_granted_by() {
        test_parser!(
            source = "granted by public",
            parser = granted_by,
            expected = RoleSpec::Public
        )
    }
}

use pg_ast::GrantOption;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Grant;
use pg_lexer::Keyword::Granted;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::OptionKw;
use pg_lexer::Keyword::With;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_sink_ast::RoleSpec;
use pg_sink_combinators::role_spec;
