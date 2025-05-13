/// Includes: `AlterEventTrigStmt`
pub(super) fn alter_event_trigger_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER EVENT TRIGGER ColId enable_trigger
        ALTER EVENT TRIGGER ColId OWNER TO RoleSpec
        ALTER EVENT TRIGGER ColId RENAME TO ColId
    */

    sequence!(
        Event.and(Trigger).skip(),
        col_id(),
    ).chain(match_first_with_state!(|(_, trigger), stream| {
        { enable_trigger() } => (state) {
            AlterEventTrigStmt::new(trigger, state).into()
        },
        {
            Owner.and(To)
                .and_right(role_spec())
        } => (new_owner) {
            AlterOwnerStmt::new(
                AlterOwnerTarget::EventTrigger(trigger),
                new_owner
            ).into()
        },
        {
            Rename.and(To)
                .and_right(col_id())
        } => (new_name) {
            RenameStmt::new(
                RenameTarget::EventTrigger(trigger),
                new_name
            ).into()
        }
    }))
}

fn enable_trigger() -> impl Combinator<Output = EventTriggerState> {

    /*
        ENABLE_P
      | ENABLE_P REPLICA
      | ENABLE_P ALWAYS
      | DISABLE_P
    */

    match_first! {
        Disable.map(|_| Disabled),
        sequence!(
            Enable.skip(),
            or(
                Replica.map(|_| FiresOnReplica),
                Always.map(|_| FiresAlways)
            )
            .optional()
        ).map(|(_, enable)|
            enable.unwrap_or(FiresOnOrigin)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use postgres_parser_ast::RoleSpec;
    use test_case::test_case;

    #[test]
    fn test_alter_enable() {
        let mut stream = TokenStream::new("event trigger trigger_name enable", DEFAULT_CONFIG);

        let expected = AlterEventTrigStmt::new("trigger_name", FiresOnOrigin);

        assert_eq!(Ok(expected.into()), alter_event_trigger_stmt().parse(&mut stream));
    }

    #[test]
    fn test_alter_owner() {
        let mut stream = TokenStream::new("event trigger trigger_name owner to public", DEFAULT_CONFIG);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::EventTrigger("trigger_name".into()),
            RoleSpec::Public,
        );

        assert_eq!(Ok(expected.into()), alter_event_trigger_stmt().parse(&mut stream));
    }

    #[test]
    fn test_alter_rename() {
        let mut stream = TokenStream::new("event trigger trigger_name rename to another_trigger", DEFAULT_CONFIG);

        let expected = RenameStmt::new(
            RenameTarget::EventTrigger("trigger_name".into()),
            "another_trigger"
        );

        assert_eq!(Ok(expected.into()), alter_event_trigger_stmt().parse(&mut stream));
    }

    #[test_case("disable", Disabled)]
    #[test_case("enable", FiresOnOrigin)]
    #[test_case("enable replica", FiresOnReplica)]
    #[test_case("enable always", FiresAlways)]
    fn test_enable_trigger(source: &str, expected: EventTriggerState) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), enable_trigger().parse(&mut stream));
    }
}

use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::role_spec;
use postgres_parser_ast::AlterEventTrigStmt;
use postgres_parser_ast::AlterOwnerStmt;
use postgres_parser_ast::AlterOwnerTarget;
use postgres_parser_ast::EventTriggerState;
use postgres_parser_ast::EventTriggerState::Disabled;
use postgres_parser_ast::EventTriggerState::FiresAlways;
use postgres_parser_ast::EventTriggerState::FiresOnOrigin;
use postgres_parser_ast::EventTriggerState::FiresOnReplica;
use postgres_parser_ast::RawStmt;
use postgres_parser_ast::RenameStmt;
use postgres_parser_ast::RenameTarget;
use postgres_parser_lexer::Keyword::Always;
use postgres_parser_lexer::Keyword::Disable;
use postgres_parser_lexer::Keyword::Enable;
use postgres_parser_lexer::Keyword::Event;
use postgres_parser_lexer::Keyword::Owner;
use postgres_parser_lexer::Keyword::Rename;
use postgres_parser_lexer::Keyword::Replica;
use postgres_parser_lexer::Keyword::To;
use postgres_parser_lexer::Keyword::Trigger;
