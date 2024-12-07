/// Alias: `AlterDatabaseStmt`
pub(super) fn alter_database_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER DATABASE ColId (
              REFRESH COLLATION VERSION_P  => AlterDatabaseRefreshCollStmt
            | OWNER TO RoleSpec            => AlterOwnerStmt
            | RENAME TO ColId              => RenameStmt
            | SET TABLESPACE ColId         => AlterDatabaseStmt /* TODO */
            | SET set_rest                 => AlterDatabaseSetStmt (SetResetClause) /* TODO */
            | VariableResetStmt            => AlterDatabaseSetStmt (SetResetClause) /* TODO */
            | WITH createdb_opt_list       => AlterDatabaseStmt
            | createdb_opt_list            => AlterDatabaseStmt
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
                or(
                    With.and_right(alterdb_opt_list()),
                    alterdb_opt_list()
                )
            } => (options) {
                AlterDatabaseStmt::new(name, options).into()
            },
            // TODO
        }})
}

fn alterdb_opt_list() -> impl Combinator<Output = Vec<AlterdbOption>> {

    many(alterdb_opt_item())
}

fn alterdb_opt_item() -> impl Combinator<Output = AlterdbOption> {

    /*
          alterdb_opt_name ( '=' )? DEFAULT
        | alterdb_opt_name ( '=' )? opt_boolean_or_string
        | alterdb_opt_name ( '=' )? signed_number
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
    use crate::parser::tests::DEFAULT_CONFIG;
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
            "this_db".into()
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test_case("database the_db_name with ALLOW_CONNECTIONS default CONNECTION LIMIT = +5 IS_TEMPLATE false")]
    #[test_case("database the_db_name ALLOW_CONNECTIONS = default CONNECTION LIMIT 5 IS_TEMPLATE = false")]
    fn test_opt_list(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_database_stmt().parse(&mut stream);

        let expected = AlterDatabaseStmt::new(
            "the_db_name".into(),
            vec![
                AlterdbOption::new(AllowConnections, CreatedbOptionValue::Default),
                AlterdbOption::new(ConnectionLimit, 5),
                AlterdbOption::new(IsTemplate, false),
            ]
        );

        assert_eq!(Ok(expected.into()), actual);
    }
}

use crate::lexer::Keyword::Collation;
use crate::lexer::Keyword::Connection;
use crate::lexer::Keyword::Database;
use crate::lexer::Keyword::Limit;
use crate::lexer::Keyword::Owner;
use crate::lexer::Keyword::Refresh;
use crate::lexer::Keyword::Rename;
use crate::lexer::Keyword::To;
use crate::lexer::Keyword::Version;
use crate::lexer::Keyword::With;
use crate::lexer::OperatorKind::Equals;
use crate::parser::ast_node::AlterDatabaseStmt;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterOwnerTarget;
use crate::parser::ast_node::AlterdbOption;
use crate::parser::ast_node::AlterdbOptionKind;
use crate::parser::ast_node::AlterdbOptionKind::AllowConnections;
use crate::parser::ast_node::AlterdbOptionKind::ConnectionLimit;
use crate::parser::ast_node::AlterdbOptionKind::IsTemplate;
use crate::parser::ast_node::AlterdbOptionKind::Unknown;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RawStmt::AlterDatabaseRefreshCollStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::RenameTarget;
use crate::parser::col_id;
use crate::parser::combinators::identifier;
use crate::parser::combinators::many;
use crate::parser::combinators::match_first;
use crate::parser::combinators::match_first_with_state;
use crate::parser::combinators::or;
use crate::parser::combinators::sequence;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::role_parsers::role_spec;
use crate::parser::stmt::create_stmt::createdb_opt_value;
