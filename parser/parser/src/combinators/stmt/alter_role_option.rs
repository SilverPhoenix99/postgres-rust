/// Alias: `AlterOptRoleList`
pub(super) fn alter_role_options() -> impl Combinator<Output = Option<Vec<AlterRoleOption>>> {

    parser(|stream|
        many!(alter_role_option().parse(stream))
            .optional()
            .map_err(ScanErr)
    )
}

/// Alias: `AlterOptRoleElem`
pub(super) fn alter_role_option() -> impl Combinator<Output = AlterRoleOption> {

    /*
          PASSWORD SCONST
        | PASSWORD NULL
        | ENCRYPTED PASSWORD SCONST
        | UNENCRYPTED PASSWORD SCONST
        | CONNECTION LIMIT SignedIconst
        | VALID UNTIL SCONST
        | USER role_list
        | INHERIT
        | IDENT
    */

    parser(|stream| {
        choice!(stream,
            {
                password_option(stream)
            },
            {
                seq!(
                    Connection.parse(stream),
                    Limit.parse(stream),
                    signed_i32_literal().parse(stream),
                )
                .map(|(.., limit)| ConnectionLimit(limit))
            },
            {
                seq!(
                    Valid.parse(stream),
                    Until.parse(stream),
                    string(stream)
                )
                .map(|(.., valid)| ValidUntil(valid))
            },
            {
                // Supported but not documented for roles, for use by ALTER GROUP.
                seq!(
                    User.parse(stream),
                    role_list().parse(stream)
                )
                .map(|(_, role_list)| RoleMembers(role_list))
            },
            {
                Kw::Inherit
                    .parse(stream)
                    .map(|_| Inherit(true))
            },
            {
                ident_option(stream)
            }
        )
    })
}

fn password_option(stream: &mut TokenStream) -> Result<AlterRoleOption> {

    choice!(stream,
        {
            seq!(
                Kw::Password.parse(stream),
                choice!(stream,
                    string(stream).map(Some),
                    Null.parse(stream).map(|_| None)
                )
            )
            .map(|(_, pw)| Password(pw))
        },
        {
            /*
             * These days, passwords are always stored in encrypted
             * form, so there is no difference between PASSWORD and
             * ENCRYPTED PASSWORD.
             */
            seq!(
                Encrypted.parse(stream),
                Kw::Password.parse(stream),
                string(stream)
            )
            .map(|(.., pw)|
                Password(Some(pw))
            )
        },
        {
            let (_, loc) = located!(stream,
                seq!(
                    Unencrypted.parse(stream),
                    Kw::Password.parse(stream),
                    string(stream)
                )
            )?;

            let err = LocatedError::new(UnencryptedPassword, loc);
            Err::<AlterRoleOption, _>(ScanErr(err))
        }
    )
}

fn ident_option(stream: &mut TokenStream) -> Result<AlterRoleOption> {

    let (ident, loc) = located!(stream, identifier(stream))?;

    match &*ident {
        "superuser" => Ok(SuperUser(true)),
        "nosuperuser" => Ok(SuperUser(false)),
        "createrole" => Ok(CreateRole(true)),
        "nocreaterole" => Ok(CreateRole(false)),
        "replication" => Ok(IsReplication(true)),
        "noreplication" => Ok(IsReplication(false)),
        "createdb" => Ok(CreateDatabase(true)),
        "nocreatedb" => Ok(CreateDatabase(false)),
        "login" => Ok(CanLogin(true)),
        "nologin" => Ok(CanLogin(false)),
        "bypassrls" => Ok(BypassRls(true)),
        "nobypassrls" => Ok(BypassRls(false)),
        // Note that INHERIT is a keyword, so it's handled by main parser,
        // but NOINHERIT is handled here.
        "noinherit" => Ok(Inherit(false)),
        _ => {
            let kind = UnrecognizedRoleOption(ident);
            let err = LocatedError::new(kind, loc);
            Err(ScanErr(err))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::RoleSpec::Public;
    use test_case::test_case;

    #[test]
    fn test_alter_role_options() {
        test_parser!(
            source = "inherit password null",
            parser = alter_role_options(),
            expected = Some(vec![Inherit(true), Password(None)])
        )
    }

    #[test_case("password null", Password(None))]
    #[test_case("connection limit 5", ConnectionLimit(5))]
    #[test_case("valid until 'tomorrow'", ValidUntil("tomorrow".into()))]
    #[test_case("user public", RoleMembers(vec![Public]))]
    #[test_case("inherit", Inherit(true))]
    #[test_case("noinherit", Inherit(false))]
    fn test_alter_role_option(source: &str, expected: AlterRoleOption) {
        test_parser!(source, alter_role_option(), expected)
    }

    #[test_case("password 'password1'", Some("password1".into()))]
    #[test_case("password null", None)]
    #[test_case("encrypted password 'epw123'", Some("epw123".into()))]
    fn test_password_option(source: &str, expected: Option<Box<str>>) {
        test_parser!(v2, source, password_option, Password(expected))
    }

    #[test_case("superuser", SuperUser(true))]
    #[test_case("nosuperuser", SuperUser(false))]
    #[test_case("createrole", CreateRole(true))]
    #[test_case("nocreaterole", CreateRole(false))]
    #[test_case("replication", IsReplication(true))]
    #[test_case("noreplication", IsReplication(false))]
    #[test_case("createdb", CreateDatabase(true))]
    #[test_case("nocreatedb", CreateDatabase(false))]
    #[test_case("login", CanLogin(true))]
    #[test_case("nologin", CanLogin(false))]
    #[test_case("bypassrls", BypassRls(true))]
    #[test_case("nobypassrls", BypassRls(false))]
    #[test_case("noinherit", Inherit(false))]
    fn test_ident_option(source: &str, expected: AlterRoleOption) {
        test_parser!(v2, source, ident_option, expected)
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_list;
use crate::combinators::signed_i32_literal;
use crate::result::Optional;
use crate::scan::Error::ScanErr;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::AlterRoleOption;
use pg_ast::AlterRoleOption::BypassRls;
use pg_ast::AlterRoleOption::CanLogin;
use pg_ast::AlterRoleOption::ConnectionLimit;
use pg_ast::AlterRoleOption::CreateDatabase;
use pg_ast::AlterRoleOption::CreateRole;
use pg_ast::AlterRoleOption::Inherit;
use pg_ast::AlterRoleOption::IsReplication;
use pg_ast::AlterRoleOption::Password;
use pg_ast::AlterRoleOption::RoleMembers;
use pg_ast::AlterRoleOption::SuperUser;
use pg_ast::AlterRoleOption::ValidUntil;
use pg_elog::parser::Error::UnencryptedPassword;
use pg_elog::parser::Error::UnrecognizedRoleOption;
use pg_elog::LocatedError;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Connection;
use pg_lexer::Keyword::Encrypted;
use pg_lexer::Keyword::Limit;
use pg_lexer::Keyword::Null;
use pg_lexer::Keyword::Unencrypted;
use pg_lexer::Keyword::Until;
use pg_lexer::Keyword::User;
use pg_lexer::Keyword::Valid;
