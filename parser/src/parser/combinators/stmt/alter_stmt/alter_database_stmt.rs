enum SetOption {
    Tablespace(Str),
    SetRest(SetRest),
}

/// Alias: `AlterDatabaseStmt`
pub(super) fn alter_database_stmt() -> impl Combinator<Output = RawStmt> {

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
        .and_right(col_id())
        .chain(match_first_with_state! {|name, stream| {
            {
                sequence!(Refresh, Collation, Version)
            } => (_) {
                AlterDatabaseRefreshCollStmt(name)
            },
            {
                Owner.and(To).and_right(role_spec())
            } => (new_owner) {
                AlterOwnerStmt::new(
                    AlterOwnerTarget::Database(name),
                    new_owner
                ).into()
            },
            {
                Rename.and(To).and_right(col_id())
            } => (new_name) {
                RenameStmt::new(
                    RenameTarget::Database(name),
                    new_name
                ).into()
            },
            {
                Set.and_right(or(
                    Kw::Tablespace.and_right(col_id()).map(SetOption::Tablespace),
                    set_rest().map(SetOption::SetRest)
                ))
            } => (set_option) {
                match set_option {
                    SetOption::Tablespace(tablespace) => {
                        let option = AlterdbOption::new(Tablespace, tablespace);
                        AlterDatabaseStmt::new(name, vec![option]).into()
                    }
                    SetOption::SetRest(set) => {
                        let option = SetResetClause::Set(set);
                        AlterDatabaseSetStmt::new(name, option).into()
                    }
                }
            },
            {
                reset_stmt()
            } => (variable_target) {
                let option = SetResetClause::Reset(variable_target);
                AlterDatabaseSetStmt::new(name, option).into()
            },
            {
                or(
                    With.and_right(alterdb_opt_list()),
                    alterdb_opt_list()
                )
            } => (options) {
                AlterDatabaseStmt::new(name, options).into()
            },
        }})
}

fn alterdb_opt_list() -> impl Combinator<Output = Vec<AlterdbOption>> {

    many(alterdb_opt_item())
}

fn alterdb_opt_item() -> impl Combinator<Output = AlterdbOption> {

    /*
          alterdb_opt_name ( '=' )? DEFAULT
        | alterdb_opt_name ( '=' )? var_value
    */

    sequence!(
        alterdb_opt_name(),
        Equals.optional().skip(),
        createdb_opt_value()
    ).map(|(kind, _, value)|
        AlterdbOption::new(kind, value)
    )
}

fn alterdb_opt_name() -> impl Combinator<Output = AlterdbOptionKind> {

    match_first! {
        Connection.and(Limit).map(|_| ConnectionLimit),
        Kw::Tablespace.map(|_| Tablespace),
        identifier().map(|ident| match ident.as_ref() {
            "allow_connections" => AllowConnections,
            "is_template" => IsTemplate,
            _ => Unknown(ident)
        })
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::parser::ast_node::CreatedbOptionValue;
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::ast_node::VariableTarget;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test]
    fn test_refresh_collation_version() {
        let source = "database db_name refresh collation version";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_database_stmt().parse(&mut stream);

        let expected = AlterDatabaseRefreshCollStmt("db_name".into());
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_alter_owner() {
        let source = "database db_name owner to public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_database_stmt().parse(&mut stream);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::Database("db_name".into()),
            RoleSpec::Public
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_rename() {
        let source = "database db_name rename to this_db";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_database_stmt().parse(&mut stream);

        let expected = RenameStmt::new(
            RenameTarget::Database("db_name".into()),
            "this_db"
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_set_tablespace() {
        let source = "database db_name set tablespace some_name";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_database_stmt().parse(&mut stream);

        let expected = AlterDatabaseStmt::new(
            "db_name",
            vec![AlterdbOption::new(Tablespace, "some_name")]
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_set_rest() {
        let source = "database db_name set transaction snapshot 'tx'";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_database_stmt().parse(&mut stream);

        let expected = AlterDatabaseSetStmt::new(
            "db_name",
            SetResetClause::Set(SetRest::TransactionSnapshot("tx".into())),
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_reset() {
        let source = "database db_name reset time zone";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_database_stmt().parse(&mut stream);

        let expected = AlterDatabaseSetStmt::new(
            "db_name",
            SetResetClause::Reset(VariableTarget::TimeZone)
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test_case("database the_db_name with ALLOW_CONNECTIONS default CONNECTION LIMIT = +5 IS_TEMPLATE false TABLESPACE = tbspace")]
    #[test_case("database the_db_name ALLOW_CONNECTIONS = default CONNECTION LIMIT 5 IS_TEMPLATE = false TABLESPACE tbspace")]
    fn test_opt_list(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_database_stmt().parse(&mut stream);

        let expected = AlterDatabaseStmt::new(
            "the_db_name",
            vec![
                AlterdbOption::new(AllowConnections, CreatedbOptionValue::Default),
                AlterdbOption::new(ConnectionLimit, 5),
                AlterdbOption::new(IsTemplate, false),
                AlterdbOption::new(Tablespace, "tbspace")
            ]
        );

        assert_eq!(Ok(expected.into()), actual);
    }
}

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::Collation;
use crate::lexer::Keyword::Connection;
use crate::lexer::Keyword::Database;
use crate::lexer::Keyword::Limit;
use crate::lexer::Keyword::Owner;
use crate::lexer::Keyword::Refresh;
use crate::lexer::Keyword::Rename;
use crate::lexer::Keyword::Set;
use crate::lexer::Keyword::To;
use crate::lexer::Keyword::Version;
use crate::lexer::Keyword::With;
use crate::lexer::OperatorKind::Equals;
use crate::parser::ast_node::AlterDatabaseSetStmt;
use crate::parser::ast_node::AlterDatabaseStmt;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterOwnerTarget;
use crate::parser::ast_node::AlterdbOption;
use crate::parser::ast_node::AlterdbOptionKind;
use crate::parser::ast_node::AlterdbOptionKind::AllowConnections;
use crate::parser::ast_node::AlterdbOptionKind::ConnectionLimit;
use crate::parser::ast_node::AlterdbOptionKind::IsTemplate;
use crate::parser::ast_node::AlterdbOptionKind::Tablespace;
use crate::parser::ast_node::AlterdbOptionKind::Unknown;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RawStmt::AlterDatabaseRefreshCollStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::RenameTarget;
use crate::parser::ast_node::SetResetClause;
use crate::parser::ast_node::SetRest;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::identifier;
use crate::parser::combinators::foundation::many;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::role_spec;
use crate::parser::combinators::stmt::createdb_opt_value;
use crate::parser::combinators::stmt::reset_stmt::reset_stmt;
use crate::parser::combinators::stmt::set_rest::set_rest;
use postgres_basics::Str;
