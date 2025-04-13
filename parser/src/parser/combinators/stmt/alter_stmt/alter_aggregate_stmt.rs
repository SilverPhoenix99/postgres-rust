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
                    .and_right(col_id())
            } => (new_name) {
                RenameStmt::new(
                    RenameTarget::Aggregate(aggregate),
                    new_name
                ).into()
            },
            {
                Set.and(Schema)
                    .and_right(col_id())
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
    use crate::parser::ast_node::AggregateWithArgs;
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::tests::test_parser;

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

use crate::lexer::Keyword::Aggregate;
use crate::lexer::Keyword::Owner;
use crate::lexer::Keyword::Rename;
use crate::lexer::Keyword::Schema;
use crate::lexer::Keyword::Set;
use crate::lexer::Keyword::To;
use crate::parser::ast_node::AlterObjectSchemaStmt;
use crate::parser::ast_node::AlterObjectSchemaTarget;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterOwnerTarget;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::RenameTarget;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::role::role_spec;
use crate::parser::combinators::stmt::aggregate_with_argtypes;
