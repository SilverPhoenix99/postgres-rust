pub(super) fn role_list() -> impl Combinator<Output = Vec<RoleSpec>> {

    /*
        role_spec ( ',' role_spec )*
    */

    enclosure! { many_sep(Comma, role_spec()) }
}

/// Alias: `RoleId`
pub(super) fn role_id() -> impl Combinator<Output = Str> {

    // Similar to role_spec, but only allows an identifier, i.e., disallows builtin roles

    located(role_spec())
        .map_result(|result| {
            let (role, loc) = result?;
            role.into_role_id()
                .map_err(|err|
                    ScanErr(PgError::new(err, loc))
                )
        })
}

/// Alias: `RoleSpec`
pub(super) fn role_spec() -> impl Combinator<Output = RoleSpec> {

    /*
        role_spec :
              NONE => Err(ReservedRoleSpec)
            | CURRENT_ROLE
            | CURRENT_USER
            | SESSION_USER
            | "public"
            | non_reserved_word
    */

    match_first! {
        CurrentRole.map(|_| RoleSpec::CurrentRole),
        CurrentUser.map(|_| RoleSpec::CurrentUser),
        SessionUser.map(|_| RoleSpec::SessionUser),

        // "none" is a ColumnName keyword, so it must be checked before the next option
        located(NoneKw).map_result(|result| match result {
            Ok((_, loc)) => Err(ScanErr(
                PgError::new(ReservedRoleSpec("none"), loc)
            )),
            Err(err) => Err(err)
        }),

        non_reserved_word().map(|ident| match ident.as_ref() {
            "public" => RoleSpec::Public,
            _ => RoleSpec::Name(ident)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::Error::NoMatch;
    use crate::scan::Result;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_elog::role_spec::Error;
    use pg_elog::role_spec::Error::ForbiddenRoleSpec;
    use pg_elog::PgErrorKind;
    use std::fmt::Debug;

    #[test]
    fn test_role_list() {
        let source = "public , CuRrEnT_rOlE,CURRENT_USER, session_user ,coalesce,xxYYzz none";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = role_list().parse(&mut stream).unwrap();

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
        assert_eq!(Ok("coalesce".into()), role_id().parse(&mut stream));
        assert_eq!(Ok("xxyyzz".into()), role_id().parse(&mut stream));

        let mut stream = TokenStream::new("none", DEFAULT_CONFIG);
        assert_err(ReservedRoleSpec("none"), role_id().parse(&mut stream));

        let mut stream = TokenStream::new("public", DEFAULT_CONFIG);
        assert_err(ReservedRoleSpec("public"), role_id().parse(&mut stream));

        let mut stream = TokenStream::new("current_role", DEFAULT_CONFIG);
        assert_err(ForbiddenRoleSpec("CURRENT_ROLE"), role_id().parse(&mut stream));

        let mut stream = TokenStream::new("current_user", DEFAULT_CONFIG);
        assert_err(ForbiddenRoleSpec("CURRENT_USER"), role_id().parse(&mut stream));

        let mut stream = TokenStream::new("session_user", DEFAULT_CONFIG);
        assert_err(ForbiddenRoleSpec("SESSION_USER"), role_id().parse(&mut stream));
    }

    #[test]
    fn test_role_spec() {
        let source = "public CuRrEnT_rOlE CURRENT_USER session_user coalesce xxyyzz";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(RoleSpec::Public), role_spec().parse(&mut stream));
        assert_eq!(Ok(RoleSpec::CurrentRole), role_spec().parse(&mut stream));
        assert_eq!(Ok(RoleSpec::CurrentUser), role_spec().parse(&mut stream));
        assert_eq!(Ok(RoleSpec::SessionUser), role_spec().parse(&mut stream));
        assert_eq!(Ok(RoleSpec::Name("coalesce".into())), role_spec().parse(&mut stream));
        assert_eq!(Ok(RoleSpec::Name("xxyyzz".into())), role_spec().parse(&mut stream));

        let mut stream = TokenStream::new("collate", DEFAULT_CONFIG);
        assert_matches!(role_spec().parse(&mut stream), Err(NoMatch(_)));

        let mut stream = TokenStream::new("none", DEFAULT_CONFIG);
        assert_err(ReservedRoleSpec("none"), role_spec().parse(&mut stream));
    }

    fn assert_err<T: Debug>(expected: Error, actual: Result<T>) {
        assert_matches!(actual, Err(ScanErr(_)));
        let ScanErr(actual) = actual.unwrap_err() else {
            unreachable!("already checked for Err(ScanErr(_))");
        };

        let expected = PgErrorKind::RoleSpecError(expected);
        assert_eq!(&expected, actual.source());
    }
}

use crate::combinators::foundation::enclosure;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::non_reserved_word;
use crate::scan::Error::ScanErr;
use pg_ast::RoleSpec;
use pg_basics::Str;
use pg_elog::role_spec::Error::ReservedRoleSpec;
use pg_elog::PgError;
use pg_lexer::Keyword::CurrentRole;
use pg_lexer::Keyword::CurrentUser;
use pg_lexer::Keyword::NoneKw;
use pg_lexer::Keyword::SessionUser;
use pg_lexer::OperatorKind::Comma;
