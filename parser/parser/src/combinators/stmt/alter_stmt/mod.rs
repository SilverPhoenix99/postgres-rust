mod alter_aggregate_stmt;
mod alter_collation_stmt;
mod alter_conversion_stmt;
mod alter_database_stmt;
mod alter_default_privileges_stmt;
mod alter_event_trigger_stmt;
mod alter_extension_stmt;
mod alter_function_stmt;
mod alter_generic_option;
mod alter_group_stmt;
mod alter_language_stmt;
mod alter_large_object_stmt;
mod alter_system_stmt;
mod alter_user_stmt;
mod set_reset_clause;

pub(super) fn alter_stmt(stream: &mut TokenStream) -> Result<RawStmt> {

    Alter.and_right(choice!(
        alter_aggregate_stmt,
        alter_collation_stmt,
        alter_conversion_stmt(),
        alter_database_stmt(),
        alter_default_privileges_stmt(),
        alter_event_trigger_stmt(),
        alter_extension_stmt(),
        alter_function_stmt(),
        alter_group_stmt(),
        alter_language_stmt(),
        alter_large_object_stmt(),
        alter_system_stmt(),
        alter_user_stmt()
    ))
        .parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("alter aggregate aggregate_name(*) owner to current_user")]
    #[test_case("alter collation some_name refresh version")]
    #[test_case("alter conversion some_conversion rename to new_conversion")]
    #[test_case("alter database the_db refresh collation version")]
    #[test_case("alter default privileges in schema some_schema grant all on tables to public")]
    #[test_case("alter event trigger some_trigger owner to current_user")]
    #[test_case("alter extension foo set schema some_schema")]
    #[test_case("alter function some_function() owner to current_user")]
    #[test_case("alter group some_group rename to new_group_name")]
    #[test_case("alter language lang owner to session_user")]
    #[test_case("alter large object -127 owner to public")]
    #[test_case("alter system reset all")]
    #[test_case("alter user public")]
    fn test_alter(source: &str) {

        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_stmt(&mut stream);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            "expected Ok(_) for {source:?} but actually got {actual:?}"
        );
    }
}

use self::{
    alter_aggregate_stmt::alter_aggregate_stmt,
    alter_collation_stmt::alter_collation_stmt,
    alter_conversion_stmt::alter_conversion_stmt,
    alter_database_stmt::alter_database_stmt,
    alter_default_privileges_stmt::alter_default_privileges_stmt,
    alter_event_trigger_stmt::alter_event_trigger_stmt,
    alter_extension_stmt::alter_extension_stmt,
    alter_function_stmt::alter_function_stmt,
    alter_generic_option::alter_generic_options,
    alter_group_stmt::alter_group_stmt,
    alter_language_stmt::alter_language_stmt,
    alter_large_object_stmt::alter_large_object_stmt,
    alter_system_stmt::alter_system_stmt,
    alter_user_stmt::alter_user_stmt,
    set_reset_clause::set_reset_clause
};
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::foundation::choice;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Alter;
