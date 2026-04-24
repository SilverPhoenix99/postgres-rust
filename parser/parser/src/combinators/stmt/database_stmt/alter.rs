/// Alias: `AlterDatabaseStmt`
pub(in crate::combinators::stmt) fn alter_database_stmt(ctx: &mut ParserContext) -> scan::Result<DatabaseStmt> {

    /*
        ALTER DATABASE ColId (
              REFRESH COLLATION VERSION  => AlterDatabaseRefreshCollStmt
            | OWNER TO RoleSpec          => AlterOwnerStmt
            | RENAME TO ColId            => RenameStmt
            | SET TABLESPACE ColId       => AlterDatabaseStmt
            | SET set_rest               => AlterDatabaseSetStmt (SetResetClause)
            | VariableResetStmt          => AlterDatabaseSetStmt (SetResetClause)
            | ( WITH )? alterdb_opt_list => AlterDatabaseStmt
        )

        NB: The RHS are the struct names in PG-C.
    */

    let (_, db_name, change) = seq!(Database, col_id, change).parse(ctx)?;

    let stmt = DatabaseStmt::new(db_name, change);

    Ok(stmt)
}

fn change(ctx: &mut ParserContext) -> scan::Result<DatabaseStmtOption> {
    alt!(
        refresh_collation_version,
        change_owner,
        rename,
        set_option,
        reset_stmt.map(Reset),
        seq!(With.optional(), alterdb_opt_list)
            .map(|(_, options)| AlterOptions(options)),
    ).parse(ctx)
}

fn refresh_collation_version(ctx: &mut ParserContext) -> scan::Result<DatabaseStmtOption> {

    seq!(Refresh, Collation, Version).parse(ctx)?;

    Ok(RefreshCollationVersion)
}

fn change_owner(ctx: &mut ParserContext) -> scan::Result<DatabaseStmtOption> {

    let (.., new_owner) = seq!(Owner, To, role_spec).parse(ctx)?;

    Ok(AlterOwner { new_owner })
}

fn rename(ctx: &mut ParserContext) -> scan::Result<DatabaseStmtOption> {

    let (.., new_name) = seq!(Kw::Rename, To, col_id).parse(ctx)?;

    Ok(Rename { new_name })
}

fn set_option(ctx: &mut ParserContext) -> scan::Result<DatabaseStmtOption> {

    let (_, change) = seq!(
        Kw::Set,
        alt!(
            seq!(Kw::Tablespace, col_id)
                .map(|(_, tablespace)| {
                    let option = AlterdbOption::new(Tablespace, tablespace);
                    AlterOptions(vec![option])
                }),
            set_rest
                .map(Set)
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
    use super::*;
    use crate::test_parser;
    use pg_ast::CreatedbOptionValue;
    use pg_ast::RoleSpec;
    use pg_ast::SetRest::TransactionSnapshot;
    use pg_ast::VariableTarget::TimeZone;
    use test_case::test_matrix;

    #[test_matrix("database db_name refresh collation version" => Ok(
        DatabaseStmt::new("db_name", RefreshCollationVersion)
    ))]
    #[test_matrix("database db_name owner to public" => Ok(
        DatabaseStmt::new("db_name",
            AlterOwner { new_owner: RoleSpec::Public }
        )
    ))]
    #[test_matrix("database db_name rename to this_db" => Ok(
        DatabaseStmt::new("db_name",
            Rename { new_name: "this_db".into() }
        )
    ))]
    #[test_matrix("database db_name set tablespace some_name" => Ok(
        DatabaseStmt::new("db_name",
            AlterOptions(vec![
                AlterdbOption::new(Tablespace, "some_name")
            ])
        )
    ))]
    #[test_matrix("database db_name set transaction snapshot 'tx'" => Ok(
        DatabaseStmt::new("db_name",
            Set(TransactionSnapshot("tx".into()))
        )
    ))]
    #[test_matrix("database db_name reset time zone" => Ok(
        DatabaseStmt::new("db_name",
            Reset(TimeZone)
        )
    ))]
    #[test_matrix("database the_db_name with ALLOW_CONNECTIONS default CONNECTION LIMIT = +5 IS_TEMPLATE false TABLESPACE = tbspace" => Ok(
        DatabaseStmt::new("the_db_name",
            AlterOptions(vec![
                AlterdbOption::new(AllowConnections, CreatedbOptionValue::Default),
                AlterdbOption::new(ConnectionLimit, 5),
                AlterdbOption::new(IsTemplate, false),
                AlterdbOption::new(Tablespace, "tbspace")
            ])
        )
    ))]
    #[test_matrix("database the_db_name ALLOW_CONNECTIONS = default CONNECTION LIMIT 5 IS_TEMPLATE = false TABLESPACE tbspace" => Ok(
        DatabaseStmt::new("the_db_name",
            AlterOptions(vec![
                AlterdbOption::new(AllowConnections, CreatedbOptionValue::Default),
                AlterdbOption::new(ConnectionLimit, 5),
                AlterdbOption::new(IsTemplate, false),
                AlterdbOption::new(Tablespace, "tbspace")
            ])
        )
    ))]
    fn test_alter_database_stmt(source: &str) -> scan::Result<DatabaseStmt> {
        test_parser!(source, alter_database_stmt)
    }
}

use crate::alt;
use crate::combinators::col_id;
use crate::combinators::core::identifier;
use crate::combinators::core::Combinator;
use crate::combinators::role_spec;
use crate::combinators::stmt::database_stmt::createdb_opt_value;
use crate::combinators::stmt::reset_stmt;
use crate::combinators::stmt::set_rest;
use crate::many;
use crate::seq;
use crate::ParserContext;
use pg_ast::AlterdbOption;
use pg_ast::AlterdbOptionKind;
use pg_ast::AlterdbOptionKind::AllowConnections;
use pg_ast::AlterdbOptionKind::ConnectionLimit;
use pg_ast::AlterdbOptionKind::IsTemplate;
use pg_ast::AlterdbOptionKind::Tablespace;
use pg_ast::AlterdbOptionKind::Unknown;
use pg_ast::DatabaseStmt;
use pg_ast::DatabaseStmtOption;
use pg_ast::DatabaseStmtOption::AlterOptions;
use pg_ast::DatabaseStmtOption::AlterOwner;
use pg_ast::DatabaseStmtOption::RefreshCollationVersion;
use pg_ast::DatabaseStmtOption::Rename;
use pg_ast::DatabaseStmtOption::Reset;
use pg_ast::DatabaseStmtOption::Set;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::Connection;
use pg_lexer::Keyword::Database;
use pg_lexer::Keyword::Limit;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Refresh;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::Version;
use pg_lexer::Keyword::With;
use pg_lexer::OperatorKind::Equals;
use pg_parser_core::scan;
