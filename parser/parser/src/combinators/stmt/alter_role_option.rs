/// Alias: `AlterOptRoleList`
pub(super) fn alter_role_options() -> impl Combinator<Output = Option<Vec<AlterRoleOption>>> {

    many!(alter_role_option())
        .optional()
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

    choice!(
        password_option,
        {
            (
                Connection,
                Limit,
                signed_i32_literal(),
            )
            .map(|(.., limit)| ConnectionLimit(limit))
        },
        {
            (Valid, Until, string)
                .map(|(.., valid)| ValidUntil(valid))
        },
        {
            // Supported but not documented for roles, for use by ALTER GROUP.
            (User, role_list)
                .right()
                .map(RoleMembers)
        },
        Kw::Inherit
            .map(|_| Inherit(true)),
        ident_option
    )
}

fn password_option(stream: &mut TokenStream) -> Result<AlterRoleOption> {

    /*
          PASSWORD SCONST
        | PASSWORD NULL
        | ENCRYPTED PASSWORD SCONST
        | UNENCRYPTED PASSWORD SCONST
    */

    let parser = choice!(
        {
            (
                Kw::Password,
                choice!(
                    string.map(Some),
                    Null.map(|_| None)
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
            (Encrypted, Kw::Password, string)
                .map(|(.., pw)|
                    Password(Some(pw))
                )
        },
        {
            located!(
                (Unencrypted, Kw::Password, string)
            )
                .map_result(|result| {
                    let (_, loc) = result?;
                    let err = LocatedError::new(UnencryptedPassword, loc);
                    Err::<AlterRoleOption, _>(ScanErr(err))
                })
        }
    );

    parser.parse(stream)
}

fn ident_option(stream: &mut TokenStream) -> Result<AlterRoleOption> {

    let (ident, loc) = located!(identifier).parse(stream)?;

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
        test_parser!(source, password_option, Password(expected))
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
        test_parser!(source, ident_option, expected)
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_list;
use crate::combinators::signed_i32_literal;
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
