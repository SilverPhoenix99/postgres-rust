/// Alias: `AlterDatabaseStmt`
pub(super) fn alter_database_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER DATABASE ColId (
              REFRESH COLLATION VERSION_P  => AlterDatabaseRefreshCollStmt
            | OWNER TO RoleSpec            => AlterOwnerStmt
            | RENAME TO ColId              => RenameStmt
            | SET TABLESPACE ColId         => AlterDatabaseStmt
            | SET set_rest                 => AlterDatabaseSetStmt (SetResetClause)
            | VariableResetStmt            => AlterDatabaseSetStmt (SetResetClause)
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
            }
            // TODO
        }})
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

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
}

use crate::lexer::Keyword::Collation;
use crate::lexer::Keyword::Database;
use crate::lexer::Keyword::Owner;
use crate::lexer::Keyword::Refresh;
use crate::lexer::Keyword::Rename;
use crate::lexer::Keyword::To;
use crate::lexer::Keyword::Version;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterOwnerTarget;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RawStmt::AlterDatabaseRefreshCollStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::RenameTarget;
use crate::parser::col_id;
use crate::parser::combinators::match_first_with_state;
use crate::parser::combinators::sequence;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::role_parsers::role_spec;
