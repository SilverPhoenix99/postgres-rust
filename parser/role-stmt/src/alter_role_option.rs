/// Alias: `AlterOptRoleList`
pub fn alter_role_options(ctx: &mut ParserContext) -> scan::Result<Vec<AlterRoleOption>> {
    many!(alter_role_option).parse(ctx)
}

/// Alias: `AlterOptRoleElem`
pub(super) fn alter_role_option(ctx: &mut ParserContext) -> scan::Result<AlterRoleOption> {

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

    alt!(
        password_option,
        seq!(Connection, Limit, signed_i32_literal)
            .map(|(.., limit)| ConnectionLimit(limit)),
        seq!(Valid, Until, string)
            .map(|(.., valid)| ValidUntil(valid)),
        // Supported but not documented for roles, for use by ALTER GROUP.
        seq!(User, role_list)
            .map(|(_, members)|
                RoleMembers { action: AddDrop::Add, members }
            ),
        Kw::Inherit.map(|_| Inherit(true)),
        ident_option
    ).parse(ctx)
}

fn password_option(ctx: &mut ParserContext) -> scan::Result<AlterRoleOption> {

    /*
          PASSWORD SCONST
        | PASSWORD NULL
        | ENCRYPTED PASSWORD SCONST
        | UNENCRYPTED PASSWORD SCONST
    */

    alt!(
        seq!(
            Kw::Password,
            alt!(
                string.map(Some),
                Null.map(|_| None)
            )
        )
            .map(|(_, pw)| Password(pw)),
        /*
         * These days, passwords are always stored in encrypted
         * form, so there is no difference between PASSWORD and
         * ENCRYPTED PASSWORD.
         */
        seq!(Encrypted, Kw::Password, string)
            .map(|(.., pw)| Password(Some(pw))),
        unencrypted_password_option
    ).parse(ctx)
}

fn unencrypted_password_option(ctx: &mut ParserContext) -> scan::Result<AlterRoleOption> {

    let (Located(_, loc), ..) = seq!(located!(Unencrypted), Kw::Password, string)
        .parse(ctx)?;

    Err(UnencryptedPassword.at_location(loc).into())
}

fn ident_option(ctx: &mut ParserContext) -> scan::Result<AlterRoleOption> {

    let Located(ident, loc) = located!(identifier).parse(ctx)?;

    let option = match &*ident {
        "superuser" => SuperUser(true),
        "nosuperuser" => SuperUser(false),
        "createrole" => CreateRole(true),
        "nocreaterole" => CreateRole(false),
        "replication" => IsReplication(true),
        "noreplication" => IsReplication(false),
        "createdb" => CreateDatabase(true),
        "nocreatedb" => CreateDatabase(false),
        "login" => CanLogin(true),
        "nologin" => CanLogin(false),
        "bypassrls" => BypassRls(true),
        "nobypassrls" => BypassRls(false),
        // Note that INHERIT is a keyword, so it's handled by main parser,
        // but NOINHERIT is handled here.
        "noinherit" => Inherit(false),
        _ => return Err(UnrecognizedRoleOption(ident).at_location(loc).into()),
    };

    Ok(option)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    #[allow(unused_imports)]
    use pg_sink_ast::RoleSpec::Public;
    use test_case::test_case;

    #[test]
    fn test_alter_role_options() {
        test_parser!(
            source = "inherit password null",
            parser = alter_role_options,
            expected = vec![Inherit(true), Password(None)]
        )
    }

    #[test_case("password null", Password(None))]
    #[test_case("connection limit 5", ConnectionLimit(5))]
    #[test_case("valid until 'tomorrow'", ValidUntil("tomorrow".into()))]
    #[test_case("user public", RoleMembers { action: AddDrop::Add, members: vec![Public] })]
    #[test_case("inherit", Inherit(true))]
    #[test_case("noinherit", Inherit(false))]
    fn test_alter_role_option(source: &str, expected: AlterRoleOption) {
        test_parser!(source, alter_role_option, expected)
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

use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::located;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_elog::parser::Error::UnencryptedPassword;
use pg_elog::parser::Error::UnrecognizedRoleOption;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Connection;
use pg_lexer::Keyword::Encrypted;
use pg_lexer::Keyword::Limit;
use pg_lexer::Keyword::Null;
use pg_lexer::Keyword::Unencrypted;
use pg_lexer::Keyword::Until;
use pg_lexer::Keyword::User;
use pg_lexer::Keyword::Valid;
use pg_parser_core::scan;
use pg_role_ast::AlterRoleOption;
use pg_role_ast::AlterRoleOption::BypassRls;
use pg_role_ast::AlterRoleOption::CanLogin;
use pg_role_ast::AlterRoleOption::ConnectionLimit;
use pg_role_ast::AlterRoleOption::CreateDatabase;
use pg_role_ast::AlterRoleOption::CreateRole;
use pg_role_ast::AlterRoleOption::Inherit;
use pg_role_ast::AlterRoleOption::IsReplication;
use pg_role_ast::AlterRoleOption::Password;
use pg_role_ast::AlterRoleOption::RoleMembers;
use pg_role_ast::AlterRoleOption::SuperUser;
use pg_role_ast::AlterRoleOption::ValidUntil;
use pg_sink_ast::AddDrop;
use pg_sink_combinators::role_list;
use pg_sink_combinators::signed_i32_literal;
