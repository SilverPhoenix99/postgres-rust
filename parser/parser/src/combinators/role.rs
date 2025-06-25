pub(super) fn role_list(stream: &mut TokenStream) -> Result<Vec<RoleSpec>> {

    /*
        role_spec ( ',' role_spec )*
    */

    many!(stream => sep = Comma, role_spec)
}

/// Alias: `RoleId`
pub(super) fn role_id(stream: &mut TokenStream) -> Result<Str> {

    // Similar to role_spec, but only allows an identifier, i.e., disallows builtin roles

    let (role, loc) = located!(stream => role_spec)?;

    role.into_role_id()
        .map_err(|err| err.at(loc).into())
}

/// Alias: `RoleSpec`
pub(super) fn role_spec(stream: &mut TokenStream) -> Result<RoleSpec> {

    /*
        role_spec :
              NONE => Err(ReservedRoleSpec)
            | CURRENT_ROLE
            | CURRENT_USER
            | SESSION_USER
            | "public"
            | non_reserved_word
    */

    choice!(parsed stream =>
        CurrentRole.map(|_| RoleSpec::CurrentRole),
        CurrentUser.map(|_| RoleSpec::CurrentUser),
        SessionUser.map(|_| RoleSpec::SessionUser),
        // "none" is a ColumnName keyword, so it must be checked before the next option
        role_none,
        non_reserved_word().map(|ident| match ident.as_ref() {
            "public" => RoleSpec::Public,
            _ => RoleSpec::Name(ident)
        })
    )
}

fn role_none(stream: &mut TokenStream) -> Result<RoleSpec> {

    let (_, loc) = located!(stream => NoneKw)?;
    let err = ReservedRoleSpec("none").at(loc);
    Err(err.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::Error::{NoMatch, ScanErr};
    use crate::scan::Result;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use core::fmt::Debug;
    use pg_ast::RoleSpec;
    use pg_elog::role_spec::Error;
    use pg_elog::role_spec::Error::ForbiddenRoleSpec;
    use pg_elog::Error::Role;

    #[test]
    fn test_role_list() {
        let source = "public , CuRrEnT_rOlE,CURRENT_USER, session_user ,coalesce,xxYYzz none";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = role_list(&mut stream).unwrap();

        let expected = [
            RoleSpec::Public,
            RoleSpec::CurrentRole,
            RoleSpec::CurrentUser,
            RoleSpec::SessionUser,
            RoleSpec::Name("coalesce".into()),
            RoleSpec::Name("xxyyzz".into()),
        ];

        assert_eq!(expected, actual.as_slice());
    }

    #[test]
    fn test_role_id() {

        let mut stream = TokenStream::new("coalesce xxyyzz", DEFAULT_CONFIG);
        assert_eq!(Ok("coalesce".into()), role_id(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), role_id(&mut stream));

        let mut stream = TokenStream::new("none", DEFAULT_CONFIG);
        assert_err(ReservedRoleSpec("none"), role_id(&mut stream));

        let mut stream = TokenStream::new("public", DEFAULT_CONFIG);
        assert_err(ReservedRoleSpec("public"), role_id(&mut stream));

        let mut stream = TokenStream::new("current_role", DEFAULT_CONFIG);
        assert_err(ForbiddenRoleSpec("CURRENT_ROLE"), role_id(&mut stream));

        let mut stream = TokenStream::new("current_user", DEFAULT_CONFIG);
        assert_err(ForbiddenRoleSpec("CURRENT_USER"), role_id(&mut stream));

        let mut stream = TokenStream::new("session_user", DEFAULT_CONFIG);
        assert_err(ForbiddenRoleSpec("SESSION_USER"), role_id(&mut stream));
    }

    #[test]
    fn test_role_spec() {
        let source = "public CuRrEnT_rOlE CURRENT_USER session_user coalesce xxyyzz";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(RoleSpec::Public), role_spec(&mut stream));
        assert_eq!(Ok(RoleSpec::CurrentRole), role_spec(&mut stream));
        assert_eq!(Ok(RoleSpec::CurrentUser), role_spec(&mut stream));
        assert_eq!(Ok(RoleSpec::SessionUser), role_spec(&mut stream));
        assert_eq!(Ok(RoleSpec::Name("coalesce".into())), role_spec(&mut stream));
        assert_eq!(Ok(RoleSpec::Name("xxyyzz".into())), role_spec(&mut stream));

        let mut stream = TokenStream::new("collate", DEFAULT_CONFIG);
        assert_matches!(role_spec(&mut stream), Err(NoMatch(_)));

        let mut stream = TokenStream::new("none", DEFAULT_CONFIG);
        assert_err(ReservedRoleSpec("none"), role_spec(&mut stream));
    }

    fn assert_err<T: Debug>(expected: Error, actual: Result<T>) {
        assert_matches!(actual, Err(ScanErr(_)));
        let ScanErr(actual) = actual.unwrap_err() else {
            unreachable!("already checked for Err(ScanErr(_))");
        };

        let expected = Role(expected);
        assert_eq!(&expected, actual.source());
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many;
use crate::combinators::foundation::Combinator;
use crate::combinators::non_reserved_word;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::RoleSpec;
use pg_basics::Str;
use pg_elog::role_spec::Error::ReservedRoleSpec;
use pg_lexer::Keyword::CurrentRole;
use pg_lexer::Keyword::CurrentUser;
use pg_lexer::Keyword::NoneKw;
use pg_lexer::Keyword::SessionUser;
use pg_lexer::OperatorKind::Comma;
