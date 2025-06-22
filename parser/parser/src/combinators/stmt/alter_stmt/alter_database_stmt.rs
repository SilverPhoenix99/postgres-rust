enum Change {
    RefreshVersion,
    Owner(RoleSpec),
    Name(Str),
    SetTablespace(Str),
    SetOption(SetRest),
    ResetOption(VariableTarget),
    Options(Vec<AlterdbOption>)
}

/// Alias: `AlterDatabaseStmt`
pub(super) fn alter_database_stmt(stream: &mut TokenStream) -> Result<RawStmt> {
    /*
        ALTER DATABASE ColId (
              REFRESH COLLATION VERSION => AlterDatabaseRefreshCollStmt
            | OWNER TO RoleSpec         => AlterOwnerStmt
            | RENAME TO ColId           => RenameStmt
            | SET TABLESPACE ColId      => AlterDatabaseStmt
            | SET set_rest              => AlterDatabaseSetStmt (SetResetClause)
            | VariableResetStmt         => AlterDatabaseSetStmt (SetResetClause)
            | WITH alterdb_opt_list     => AlterDatabaseStmt
            | alterdb_opt_list          => AlterDatabaseStmt
        )
    */

    Database
        .and_right(seq!(
            col_id,
            choice!(
                seq!(Refresh, Collation, Version)
                    .map(|_| Change::RefreshVersion),
                seq!(Owner, To, role_spec)
                    .map(|(.., new_owner)| Change::Owner(new_owner)),
                seq!(Rename, To, col_id)
                    .map(|(.., new_name)| Change::Name(new_name)),
                {
                    seq!(
                        Set,
                        choice!(
                            seq!(Kw::Tablespace, col_id)
                                .right()
                                .map(Change::SetTablespace),
                            set_rest
                                .map(Change::SetOption)
                        )
                    )
                    .right::<_, Change>()
                },
                reset_stmt()
                    .map(Change::ResetOption),
                seq!(With, alterdb_opt_list)
                    .right()
                    .map(Change::Options),
                alterdb_opt_list
                    .map(Change::Options),
            )
        ))
        .parse(stream)
        .map(|(name, change)| match change {
            Change::RefreshVersion => {
                AlterDatabaseRefreshCollStmt(name)
            }
            Change::Owner(new_owner) => {
                AlterOwnerStmt::new(
                    AlterOwnerTarget::Database(name),
                    new_owner
                ).into()
            }
            Change::Name(new_name) => {
                RenameStmt::new(
                    RenameTarget::Database(name),
                    new_name
                ).into()
            }
            Change::SetTablespace(tablespace) => {
                let option = AlterdbOption::new(Tablespace, tablespace);
                AlterDatabaseStmt::new(name, vec![option]).into()
            }
            Change::SetOption(option) => {
                let option = SetResetClause::Set(option);
                AlterDatabaseSetStmt::new(name, option).into()
            }
            Change::ResetOption(option) => {
                let option = SetResetClause::Reset(option);
                AlterDatabaseSetStmt::new(name, option).into()
            }
            Change::Options(options) => {
                AlterDatabaseStmt::new(name, options).into()
            }
        })
}

fn alterdb_opt_list(stream: &mut TokenStream) -> Result<Vec<AlterdbOption>> {

    many!(alterdb_opt_item).parse(stream)
}

fn alterdb_opt_item(stream: &mut TokenStream) -> Result<AlterdbOption> {

    /*
          alterdb_opt_name ( '=' )? DEFAULT
        | alterdb_opt_name ( '=' )? var_value
    */

    seq!(
        alterdb_opt_name,
        Equals.optional(),
        createdb_opt_value()
    )
        .map(|(kind, _, value)|
            AlterdbOption::new(kind, value)
        )
        .parse(stream)
}

fn alterdb_opt_name(stream: &mut TokenStream) -> Result<AlterdbOptionKind> {

    choice! (
        Connection.and(Limit).map(|_| ConnectionLimit),
        Kw::Tablespace.map(|_| Tablespace),
        identifier.map(|ident| match ident.as_ref() {
            "allow_connections" => AllowConnections,
            "is_template" => IsTemplate,
            _ => Unknown(ident)
        })
    )
        .parse(stream)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::CreatedbOptionValue;
    use pg_ast::RoleSpec;
    use pg_ast::VariableTarget;
    use test_case::test_case;

    #[test]
    fn test_refresh_collation_version() {
        test_parser!(
            source = "database db_name refresh collation version",
            parser = alter_database_stmt,
            expected = AlterDatabaseRefreshCollStmt("db_name".into())
        )
    }

    #[test]
    fn test_alter_owner() {
        test_parser!(
            source = "database db_name owner to public",
            parser = alter_database_stmt,
            expected = AlterOwnerStmt::new(
                AlterOwnerTarget::Database("db_name".into()),
                RoleSpec::Public
            )
        )
    }

    #[test]
    fn test_rename() {
        test_parser!(
            source = "database db_name rename to this_db",
            parser = alter_database_stmt,
            expected = RenameStmt::new(
                RenameTarget::Database("db_name".into()),
                "this_db"
            )
        )
    }

    #[test]
    fn test_set_tablespace() {
        test_parser!(
            source = "database db_name set tablespace some_name",
            parser = alter_database_stmt,
            expected = AlterDatabaseStmt::new(
                "db_name",
                vec![AlterdbOption::new(Tablespace, "some_name")]
            )
        )
    }

    #[test]
    fn test_set_rest() {
        test_parser!(
            source = "database db_name set transaction snapshot 'tx'",
            parser = alter_database_stmt,
            expected = AlterDatabaseSetStmt::new(
                "db_name",
                SetResetClause::Set(SetRest::TransactionSnapshot("tx".into())),
            )
        )
    }

    #[test]
    fn test_reset() {
        test_parser!(
            source = "database db_name reset time zone",
            parser = alter_database_stmt,
            expected = AlterDatabaseSetStmt::new(
                "db_name",
                SetResetClause::Reset(VariableTarget::TimeZone)
            )
        )
    }

    #[test_case("database the_db_name with ALLOW_CONNECTIONS default CONNECTION LIMIT = +5 IS_TEMPLATE false TABLESPACE = tbspace")]
    #[test_case("database the_db_name ALLOW_CONNECTIONS = default CONNECTION LIMIT 5 IS_TEMPLATE = false TABLESPACE tbspace")]
    fn test_opt_list(source: &str) {
        let expected = AlterDatabaseStmt::new(
            "the_db_name",
            vec![
                AlterdbOption::new(AllowConnections, CreatedbOptionValue::Default),
                AlterdbOption::new(ConnectionLimit, 5),
                AlterdbOption::new(IsTemplate, false),
                AlterdbOption::new(Tablespace, "tbspace")
            ]
        );

        test_parser!(source, alter_database_stmt, expected)
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::role_spec;
use crate::combinators::stmt::createdb_opt_value;
use crate::combinators::stmt::reset_stmt::reset_stmt;
use crate::combinators::stmt::set_rest;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::AlterDatabaseSetStmt;
use pg_ast::AlterDatabaseStmt;
use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::AlterdbOption;
use pg_ast::AlterdbOptionKind;
use pg_ast::AlterdbOptionKind::AllowConnections;
use pg_ast::AlterdbOptionKind::ConnectionLimit;
use pg_ast::AlterdbOptionKind::IsTemplate;
use pg_ast::AlterdbOptionKind::Tablespace;
use pg_ast::AlterdbOptionKind::Unknown;
use pg_ast::RawStmt;
use pg_ast::RawStmt::AlterDatabaseRefreshCollStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_ast::RoleSpec;
use pg_ast::SetResetClause;
use pg_ast::SetRest;
use pg_ast::VariableTarget;
use pg_basics::Str;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::Connection;
use pg_lexer::Keyword::Database;
use pg_lexer::Keyword::Limit;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Refresh;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::Version;
use pg_lexer::Keyword::With;
use pg_lexer::OperatorKind::Equals;
