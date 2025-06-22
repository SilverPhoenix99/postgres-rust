pub(super) fn alter_aggregate_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER AGGREGATE aggregate_with_argtypes (
            OWNER TO RoleSpec => AlterOwnerStmt
            RENAME TO ColId   => RenameStmt
            SET SCHEMA ColId  => AlterObjectSchemaStmt
        )
    */

    Aggregate.and_right(aggregate_with_argtypes())
        .chain(match_first_with_state!{|aggregate, stream| {
            {
                Owner.and(To)
                    .and_right(role_spec())
            } => (new_owner) {
                AlterOwnerStmt::new(
                    AlterOwnerTarget::Aggregate(aggregate),
                    new_owner
                ).into()
            },
            {
                Rename.and(To)
                    .and_right(parser(col_id))
            } => (new_name) {
                RenameStmt::new(
                    RenameTarget::Aggregate(aggregate),
                    new_name
                ).into()
            },
            {
                Set.and(Schema)
                    .and_right(parser(col_id))
            } => (new_schema) {
                AlterObjectSchemaStmt::new(
                    AlterObjectSchemaTarget::Aggregate(aggregate),
                    new_schema
                ).into()
            }
        }})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::AggregateWithArgs;
    use pg_ast::RoleSpec;

    #[test]
    fn test_alter_owner() {
        test_parser!(
            source = "aggregate aggregate_name(*) owner to current_user",
            parser = alter_aggregate_stmt(),
            expected = AlterOwnerStmt::new(
                AlterOwnerTarget::Aggregate(
                    AggregateWithArgs::new(vec!["aggregate_name".into()], vec![], vec![])
                ),
                RoleSpec::CurrentUser
            ).into()
        )
    }

    #[test]
    fn test_rename() {
        test_parser!(
            source = "aggregate aggregate_name(*) rename to different_name",
            parser = alter_aggregate_stmt(),
            expected = RenameStmt::new(
                RenameTarget::Aggregate(
                    AggregateWithArgs::new(vec!["aggregate_name".into()], vec![], vec![])
                ),
                "different_name"
            ).into()
        )
    }

    #[test]
    fn test_alter_schema() {
        test_parser!(
            source = "aggregate aggregate_name(*) set schema new_schema",
            parser = alter_aggregate_stmt(),
            expected = AlterObjectSchemaStmt::new(
                AlterObjectSchemaTarget::Aggregate(
                    AggregateWithArgs::new(vec!["aggregate_name".into()], vec![], vec![])
                ),
                "new_schema"
            ).into()
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::match_first_with_state;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::role::role_spec;
use crate::combinators::stmt::aggregate_with_argtypes;
use pg_ast::AlterObjectSchemaStmt;
use pg_ast::AlterObjectSchemaTarget;
use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_lexer::Keyword::Aggregate;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::Schema;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::To;
