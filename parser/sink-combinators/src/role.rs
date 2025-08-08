pub fn role_list(stream: &mut TokenStream) -> scan::Result<Vec<RoleSpec>> {

    /*
        role_spec ( ',' role_spec )*
    */

    many!(sep = Comma, role_spec).parse(stream)
}

/// Alias: `RoleId`
pub fn role_id(stream: &mut TokenStream) -> scan::Result<Str> {

    // Similar to role_spec, but only allows an identifier, i.e., disallows builtin roles

    let (role, loc) = located!(role_spec).parse(stream)?;

    role.into_role_id()
        .map_err(|err| err.at(loc).into())
}

/// Alias: `RoleSpec`
pub fn role_spec(stream: &mut TokenStream) -> scan::Result<RoleSpec> {

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
    ).parse(stream)
}

fn role_none(stream: &mut TokenStream) -> scan::Result<RoleSpec> {

    let (_, loc) = located!(NoneKw).parse(stream)?;
    let err = ReservedRoleSpec { role: "none" }.at(loc);
    Err(err.into())
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
        pg_basics::Location,
        scan::Error::NoMatch,
    };

    #[test]
    fn test_role_list() {
        let source = "puBlic , CuRrEnT_rOlE,CURRENT_USER, session_user ,coalesce,xxYYzz none";
        let mut stream = TokenStream::from(source);

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

    #[test_case("coalesce" => Ok("coalesce".into()))]
    #[test_case("xxyyzz" => Ok("xxyyzz".into()))]
    #[test_case("none" => Err(ReservedRoleSpec { role: "none" }.at(Location::new(0..4, 1, 1)).into()))]
    #[test_case("public" => Err(ReservedRoleSpec { role: "public" }.at(Location::new(0..6, 1, 1)).into()))]
    #[test_case("current_role" => Err(ForbiddenRoleSpec { role: "CURRENT_ROLE" }.at(Location::new(0..12, 1, 1)).into()))]
    #[test_case("current_user" => Err(ForbiddenRoleSpec { role: "CURRENT_USER" }.at(Location::new(0..12, 1, 1)).into()))]
    #[test_case("session_user" => Err(ForbiddenRoleSpec { role: "SESSION_USER" }.at(Location::new(0..12, 1, 1)).into()))]
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
    #[test_case("none" => Err(ReservedRoleSpec { role: "none" }.at(Location::new(0..4, 1, 1)).into()))]
    fn test_role_spec(source: &str) -> scan::Result<RoleSpec> {
        test_parser!(source, role_spec)
    }
}

use crate::non_reserved_word;
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
use pg_parser_core::stream::TokenStream;
use pg_sink_ast::RoleSpec;
