mod alter_collation_stmt;
mod alter_conversion_stmt;
mod alter_database_stmt;
mod alter_default_privileges_stmt;
mod alter_event_trigger_stmt;
mod alter_group_stmt;
mod alter_language_stmt;
mod alter_large_object_stmt;
mod alter_system_stmt;
mod set_reset_clause;

pub(super) fn alter_stmt() -> impl Combinator<Output = RawStmt> {

    Alter.and_right(match_first! {
        alter_collation_stmt(),
        alter_conversion_stmt(),
        alter_database_stmt(),
        alter_default_privileges_stmt().map(From::from),
        alter_event_trigger_stmt(),
        alter_group_stmt(),
        alter_language_stmt(),
        alter_large_object_stmt(),
        alter_system_stmt().map(From::from),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("alter collation some_name refresh version")]
    #[test_case("alter conversion some_conversion rename to new_conversion")]
    #[test_case("alter database the_db refresh collation version")]
    #[test_case("alter default privileges in schema some_schema grant all on tables to public")]
    #[test_case("alter event trigger some_trigger owner to current_user")]
    #[test_case("alter group some_group rename to new_group_name")]
    #[test_case("alter language lang owner to session_user")]
    #[test_case("alter large object -127 owner to public")]
    #[test_case("alter system reset all")]
    fn test_alter(source: &str) {

        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = alter_stmt().parse(&mut stream);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            "expected Ok(_) for {source:?} but actually got {actual:?}"
        );
    }
}

use self::alter_collation_stmt::alter_collation_stmt;
use self::alter_conversion_stmt::alter_conversion_stmt;
use self::alter_database_stmt::alter_database_stmt;
use self::alter_default_privileges_stmt::alter_default_privileges_stmt;
use self::alter_event_trigger_stmt::alter_event_trigger_stmt;
use self::alter_group_stmt::alter_group_stmt;
use self::alter_language_stmt::alter_language_stmt;
use self::alter_large_object_stmt::alter_large_object_stmt;
use self::alter_system_stmt::alter_system_stmt;
use crate::lexer::Keyword::Alter;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
