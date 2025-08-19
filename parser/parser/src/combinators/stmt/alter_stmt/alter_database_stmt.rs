enum Change {
    RefreshVersion,
    Owner(RoleSpec),
    Name(Str),
    Tablespace(Str),
    SetOption(SetRest),
    ResetOption(VariableTarget),
    Options(Vec<AlterdbOption>)
}

/// Alias: `AlterDatabaseStmt`
pub(super) fn alter_database_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {
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

    let (_, name, change) = seq!(Database, col_id, change).parse(ctx)?;

    let stmt = match change {
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
        Change::Tablespace(tablespace) => {
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
    };

    Ok(stmt)
}

fn change(ctx: &mut ParserContext) -> scan::Result<Change> {
    alt!(
        refresh_collation_version,
        change_owner,
        rename,
        set_option,
        reset_stmt
            .map(Change::ResetOption),
        seq!(With, alterdb_opt_list)
            .map(|(_, options)| Change::Options(options)),
        alterdb_opt_list
            .map(Change::Options),
    ).parse(ctx)
}

fn refresh_collation_version(ctx: &mut ParserContext) -> scan::Result<Change> {

    seq!(Refresh, Collation, Version).parse(ctx)?;
    Ok(Change::RefreshVersion)
}

fn change_owner(ctx: &mut ParserContext) -> scan::Result<Change> {

    let (.., new_owner) = seq!(Owner, To, role_spec).parse(ctx)?;
    Ok(Change::Owner(new_owner))
}

fn rename(ctx: &mut ParserContext) -> scan::Result<Change> {

    let (.., new_name) = seq!(Rename, To, col_id).parse(ctx)?;
    Ok(Change::Name(new_name))
}

fn set_option(ctx: &mut ParserContext) -> scan::Result<Change> {

    let (_, change) = seq!(
        Set,
        alt!(
            seq!(Kw::Tablespace, col_id)
                .map(|(_, tablespace)| Change::Tablespace(tablespace)),
            set_rest
                .map(Change::SetOption)
        )
    ).parse(ctx)?;
    Ok(change)
}

fn alterdb_opt_list(ctx: &mut ParserContext) -> scan::Result<Vec<AlterdbOption>> {

    many!(alterdb_opt_item).parse(ctx)
}

fn alterdb_opt_item(ctx: &mut ParserContext) -> scan::Result<AlterdbOption> {

    /*
          alterdb_opt_name ( '=' )? DEFAULT
        | alterdb_opt_name ( '=' )? var_value
    */

    let (kind, _, value) = seq!(
        alterdb_opt_name,
        Equals.optional(),
        createdb_opt_value
    ).parse(ctx)?;

    let option = AlterdbOption::new(kind, value);
    Ok(option)
}

fn alterdb_opt_name(ctx: &mut ParserContext) -> scan::Result<AlterdbOptionKind> {

    alt!(
        seq!(Connection, Limit).map(|_| ConnectionLimit),
        Kw::Tablespace.map(|_| Tablespace),
        identifier.map(|ident| match ident.as_ref() {
            "allow_connections" => AllowConnections,
            "is_template" => IsTemplate,
            _ => Unknown(ident)
        })
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use pg_ast::CreatedbOptionValue;
    use pg_combinators::test_parser;
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

use crate::combinators::stmt::createdb_opt_value;
use crate::combinators::stmt::reset_stmt::reset_stmt;
use crate::combinators::stmt::set_rest;
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
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::identifier;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_generic_set_ast::SetResetClause;
use pg_generic_set_ast::SetRest;
use pg_generic_set_ast::VariableTarget;
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
use pg_parser_core::scan;
use pg_sink_ast::RoleSpec;
use pg_sink_combinators::col_id;
use pg_sink_combinators::role_spec;
