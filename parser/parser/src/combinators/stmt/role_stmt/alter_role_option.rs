/// Alias: `AlterOptRoleList`
pub(super) fn alter_role_options(ctx: &mut ParserContext) -> scan::Result<Vec<AlterRoleOption>> {
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
    use crate::test_parser;
    use pg_ast::RoleSpec::Public;
    use test_case::test_matrix;

    #[test]
    fn test_alter_role_options() {
        test_parser!(
            source = "inherit password null",
            parser = alter_role_options,
            expected = vec![Inherit(true), Password(None)]
        )
    }

    #[test_matrix("password null" => Ok(Password(None)))]
    #[test_matrix("connection limit 5" => Ok(ConnectionLimit(5)))]
    #[test_matrix("valid until 'tomorrow'" => Ok(ValidUntil("tomorrow".into())))]
    #[test_matrix("user public" => Ok(RoleMembers { action: AddDrop::Add, members: vec![Public] }))]
    #[test_matrix("inherit" => Ok(Inherit(true)))]
    #[test_matrix("noinherit" => Ok(Inherit(false)))]
    fn test_alter_role_option(source: &str) -> scan::Result<AlterRoleOption> {
        test_parser!(source, alter_role_option)
    }

    #[test_matrix("password 'password1'" => Ok(Password(Some("password1".into()))))]
    #[test_matrix("password null" => Ok(Password(None)))]
    #[test_matrix("encrypted password 'epw123'" => Ok(Password(Some("epw123".into()))))]
    fn test_password_option(source: &str) -> scan::Result<AlterRoleOption> {
        test_parser!(source, password_option)
    }

    #[test_matrix("superuser" => Ok(SuperUser(true)))]
    #[test_matrix("nosuperuser" => Ok(SuperUser(false)))]
    #[test_matrix("createrole" => Ok(CreateRole(true)))]
    #[test_matrix("nocreaterole" => Ok(CreateRole(false)))]
    #[test_matrix("replication" => Ok(IsReplication(true)))]
    #[test_matrix("noreplication" => Ok(IsReplication(false)))]
    #[test_matrix("createdb" => Ok(CreateDatabase(true)))]
    #[test_matrix("nocreatedb" => Ok(CreateDatabase(false)))]
    #[test_matrix("login" => Ok(CanLogin(true)))]
    #[test_matrix("nologin" => Ok(CanLogin(false)))]
    #[test_matrix("bypassrls" => Ok(BypassRls(true)))]
    #[test_matrix("nobypassrls" => Ok(BypassRls(false)))]
    #[test_matrix("noinherit" => Ok(Inherit(false)))]
    fn test_ident_option(source: &str) -> scan::Result<AlterRoleOption> {
        test_parser!(source, ident_option)
    }
}

use crate::alt;
use crate::combinators::core::identifier;
use crate::combinators::core::string;
use crate::combinators::core::Combinator;
use crate::combinators::role_list;
use crate::combinators::signed_i32_literal;
use crate::located;
use crate::many;
use crate::seq;
use crate::ParserContext;
use pg_ast::AddDrop;
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
use pg_basics::IntoLocated;
use pg_basics::Located;
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
