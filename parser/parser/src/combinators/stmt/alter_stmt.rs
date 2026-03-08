pub(super) fn alter_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    let (_, stmt) = seq!(
        Alter,
        alt!(
            alter_aggregate_stmt,
            alter_collation_stmt,
            alter_conversion_stmt,
            alter_database_stmt.map(From::from),
            alter_default_privileges_stmt.map(From::from),
            alter_event_trigger_stmt,
            alter_extension_stmt,
            alter_function_stmt,
            alter_group_stmt.map(From::from),
            alter_language_stmt,
            alter_large_object_stmt,
            alter_system_stmt.map(From::from),
            alter_user_stmt.map(From::from),
        )
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    // This only quickly tests that statement types aren't missing.
    // More in-depth testing is within each statement's module.
    #[test_matrix(
        [
            "alter aggregate aggregate_name(*) owner to current_user",
            "alter collation some_name refresh version",
            "alter conversion some_conversion rename to new_conversion",
            "alter database the_db refresh collation version",
            "alter default privileges in schema some_schema grant all on tables to public",
            "alter event trigger some_trigger owner to current_user",
            "alter extension foo set schema some_schema",
            "alter function some_function() owner to current_user",
            "alter group some_group rename to new_group_name",
            "alter language lang owner to session_user",
            "alter large object -127 owner to public",
            "alter system reset all",
            "alter user public",
        ]
        => matches Ok(_)
    )]
    fn test_alter_stmt(source: &str) -> scan::Result<RawStmt> {
        test_parser!(source, alter_stmt)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::stmt::alter_aggregate_stmt;
use crate::combinators::stmt::alter_collation_stmt;
use crate::combinators::stmt::alter_conversion_stmt;
use crate::combinators::stmt::alter_database_stmt;
use crate::combinators::stmt::alter_default_privileges_stmt;
use crate::combinators::stmt::alter_event_trigger_stmt;
use crate::combinators::stmt::alter_extension_stmt;
use crate::combinators::stmt::alter_function_stmt;
use crate::combinators::stmt::alter_group_stmt;
use crate::combinators::stmt::alter_language_stmt;
use crate::combinators::stmt::alter_large_object_stmt;
use crate::combinators::stmt::alter_system_stmt;
use crate::combinators::stmt::alter_user_stmt;
use crate::seq;
use crate::ParserContext;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Alter;
use pg_parser_core::scan;
