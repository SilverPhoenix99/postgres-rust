pub fn role_list(ctx: &mut ParserContext) -> scan::Result<Vec<RoleSpec>> {

    /*
        role_spec ( ',' role_spec )*
    */

    many!(sep = Comma, role_spec).parse(ctx)
}

/// Alias: `RoleId`
pub fn role_id(ctx: &mut ParserContext) -> scan::Result<Str> {

    // Similar to role_spec, but only allows an identifier, i.e., disallows builtin roles

    let Located(role, loc) = located!(role_spec).parse(ctx)?;

    let role_id = role.into_role_id()
        .map_err(|err| err.at_location(loc))?;

    Ok(role_id)
}

/// Alias: `RoleSpec`
pub fn role_spec(ctx: &mut ParserContext) -> scan::Result<RoleSpec> {

    /*
        role_spec :
              NONE => Err(ReservedRoleSpec)
            | CURRENT_ROLE
            | CURRENT_USER
            | SESSION_USER
            | "public"
            | non_reserved_word
    */

    alt!(
        CurrentRole.map(|_| RoleSpec::CurrentRole),
        CurrentUser.map(|_| RoleSpec::CurrentUser),
        SessionUser.map(|_| RoleSpec::SessionUser),
        // "none" is a ColumnName keyword, so it must be checked before the next option
        role_none,
        non_reserved_word.map(|ident| match ident.as_ref() {
            "public" => RoleSpec::Public,
            _ => RoleSpec::Name(ident)
        })
    ).parse(ctx)
}

fn role_none(ctx: &mut ParserContext) -> scan::Result<RoleSpec> {

    let Located(_, loc) = located!(NoneKw).parse(ctx)?;
    Err(ReservedRoleSpec { role: "none" }.at_location(loc).into())
}

pub trait IntoRoleId {
    fn into_role_id(self) -> role_spec::Result<Str>;
}

impl IntoRoleId for RoleSpec {
    fn into_role_id(self) -> role_spec::Result<Str> {

        let err = match self {
            Self::Name(role) => return Ok(role),
            Self::Public => ReservedRoleSpec { role: "public" },
            Self::CurrentRole => ForbiddenRoleSpec { role: "CURRENT_ROLE" },
            Self::CurrentUser => ForbiddenRoleSpec { role: "CURRENT_USER" },
            Self::SessionUser => ForbiddenRoleSpec { role: "SESSION_USER" },
        };

        Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_elog::Error::Role,
        scan::Error::{NoMatch, ScanErr},
    };

    #[test]
    fn test_role_list() {
        let source = "puBlic , CuRrEnT_rOlE,CURRENT_USER, session_user ,coalesce,xxYYzz none";
        let mut ctx = ParserContext::from(source);

        let actual = role_list(&mut ctx).unwrap();

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

    #[test_case("coalesce" => Ok("coalesce".into()))]
    #[test_case("xxyyzz" => Ok("xxyyzz".into()))]
    #[test_case("none" => matches Err(ScanErr(
        Located(Role(ReservedRoleSpec { role: "none" }), _)
    )))]
    #[test_case("public" => matches Err(ScanErr(
        Located(Role(ReservedRoleSpec { role: "public" }), _)
    )))]
    #[test_case("current_role" => matches Err(ScanErr(
        Located(Role(ForbiddenRoleSpec { role: "CURRENT_ROLE" }), _)
    )))]
    #[test_case("current_user" => matches Err(ScanErr(
        Located(Role(ForbiddenRoleSpec { role: "CURRENT_USER" }), _)
    )))]
    #[test_case("session_user" => matches Err(ScanErr(
        Located(Role(ForbiddenRoleSpec { role: "SESSION_USER" }), _)
    )))]
    fn test_role_id(source: &str) -> scan::Result<Str> {
        test_parser!(source, role_id)
    }

    #[test_case("public" => Ok(RoleSpec::Public))]
    #[test_case("CuRrEnT_rOlE" => Ok(RoleSpec::CurrentRole))]
    #[test_case("CURRENT_USER" => Ok(RoleSpec::CurrentUser))]
    #[test_case("session_user" => Ok(RoleSpec::SessionUser))]
    #[test_case("coalesce" => Ok(RoleSpec::Name("coalesce".into())))]
    #[test_case("xxyyzz" => Ok(RoleSpec::Name("xxyyzz".into())))]
    #[test_case("collate" => matches Err(NoMatch(_)))]
    #[test_case("none" => matches Err(ScanErr(
        Located(Role(ReservedRoleSpec { role: "none" }), _)
    )))]
    fn test_role_spec(source: &str) -> scan::Result<RoleSpec> {
        test_parser!(source, role_spec)
    }
}

use crate::non_reserved_word;
use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::located;
use pg_combinators::many;
use pg_combinators::Combinator;
use pg_elog::role_spec;
use pg_elog::role_spec::Error::ForbiddenRoleSpec;
use pg_elog::role_spec::Error::ReservedRoleSpec;
use pg_lexer::Keyword::CurrentRole;
use pg_lexer::Keyword::CurrentUser;
use pg_lexer::Keyword::NoneKw;
use pg_lexer::Keyword::SessionUser;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
use pg_sink_ast::RoleSpec;
