/// Alias: `AlterOptRoleList`
///
/// Post-condition: Vec **May** be empty.
pub(super) fn alter_role_options() -> impl Combinator<Output = Vec<AlterRoleOption>> {

    many(alter_role_option())
        .optional()
        .map(Option::unwrap_or_default)
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

    match_first! {
        password_option(),
        sequence!(Connection, Limit)
            .and_right(signed_i32_literal())
            .map(ConnectionLimit),
        sequence!(Valid, Until)
            .and_right(string())
            .map(ValidUntil),
        // Supported but not documented for roles, for use by ALTER GROUP.
        User.and_right(role_list())
            .map(RoleMembers),
        Kw::Inherit
            .map(|_| Inherit(true)),
        ident_option()
    }
}

fn password_option() -> impl Combinator<Output = AlterRoleOption> {

    match_first! {
        Kw::Password
            .and_right(match_first! {
                string().map(Some),
                Null.map(|_| None)
            })
            .map(Password),
        /*
         * These days, passwords are always stored in encrypted
         * form, so there is no difference between PASSWORD and
         * ENCRYPTED PASSWORD.
         */
        sequence!(Encrypted, Kw::Password)
            .and_right(string())
            .map(|pw| Password(Some(pw))),
        sequence!(located(Unencrypted.skip()), Kw::Password.skip(), string())
            .map_result(|res| match res {
                Ok(((_, loc), _, _)) => {
                    let err = ParserError::new(UnencryptedPassword, loc);
                    Err(ScanErr(err))
                },
                Err(err) => Err(err)
            })
    }
}

fn ident_option() -> impl Combinator<Output = AlterRoleOption> {

    located(identifier())
        .map_result(|ident| {

            let (ident, loc) = match ident {
                Ok(ok) => ok,
                Err(err) => return Err(err)
            };

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
                    let err = ParserError::new(kind, loc);
                    Err(ScanErr(err))
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    #[allow(unused_imports)]
    use postgres_parser_ast::RoleSpec::Public;
    use test_case::test_case;

    #[test]
    fn test_alter_role_options() {
        let source = "inherit password null";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_role_options().parse(&mut stream);

        let expected = vec![Inherit(true), Password(None)];

        assert_eq!(Ok(expected), actual);
    }

    #[test_case("password null", Password(None))]
    #[test_case("connection limit 5", ConnectionLimit(5))]
    #[test_case("valid until 'tomorrow'", ValidUntil("tomorrow".into()))]
    #[test_case("user public", RoleMembers(vec![Public]))]
    #[test_case("inherit", Inherit(true))]
    #[test_case("noinherit", Inherit(false))]
    fn test_alter_role_option(source: &str, expected: AlterRoleOption) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_role_option().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }

    #[test_case("password 'password1'", Some("password1".into()))]
    #[test_case("password null", None)]
    #[test_case("encrypted password 'epw123'", Some("epw123".into()))]
    fn test_password_option(source: &str, expected: Option<Box<str>>) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = password_option().parse(&mut stream);
        assert_eq!(Ok(Password(expected)), actual);
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
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = ident_option().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::foundation::identifier;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::role_list;
use crate::combinators::signed_i32_literal;
use crate::result::ScanErrorKind::ScanErr;
use elog::parser::ParserError;
use elog::parser::ParserErrorKind::UnencryptedPassword;
use elog::parser::ParserErrorKind::UnrecognizedRoleOption;
use postgres_parser_ast::AlterRoleOption;
use postgres_parser_ast::AlterRoleOption::BypassRls;
use postgres_parser_ast::AlterRoleOption::CanLogin;
use postgres_parser_ast::AlterRoleOption::ConnectionLimit;
use postgres_parser_ast::AlterRoleOption::CreateDatabase;
use postgres_parser_ast::AlterRoleOption::CreateRole;
use postgres_parser_ast::AlterRoleOption::Inherit;
use postgres_parser_ast::AlterRoleOption::IsReplication;
use postgres_parser_ast::AlterRoleOption::Password;
use postgres_parser_ast::AlterRoleOption::RoleMembers;
use postgres_parser_ast::AlterRoleOption::SuperUser;
use postgres_parser_ast::AlterRoleOption::ValidUntil;
use postgres_parser_lexer::Keyword as Kw;
use postgres_parser_lexer::Keyword::Connection;
use postgres_parser_lexer::Keyword::Encrypted;
use postgres_parser_lexer::Keyword::Limit;
use postgres_parser_lexer::Keyword::Null;
use postgres_parser_lexer::Keyword::Unencrypted;
use postgres_parser_lexer::Keyword::Until;
use postgres_parser_lexer::Keyword::User;
use postgres_parser_lexer::Keyword::Valid;
