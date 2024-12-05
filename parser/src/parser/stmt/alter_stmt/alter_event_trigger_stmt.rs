pub(in crate::parser) fn alter_event_trigger_stmt() -> impl Combinator<Output = RawStmt> {

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
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test]
    fn test_alter_enable() {
        let mut stream = TokenStream::new("event trigger trigger_name enable", DEFAULT_CONFIG);

        let expected = AlterEventTrigStmt::new("trigger_name".into(), FiresOnOrigin);

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
            "another_trigger".into()
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

use crate::lexer::Keyword::Always;
use crate::lexer::Keyword::Disable;
use crate::lexer::Keyword::Enable;
use crate::lexer::Keyword::Event;
use crate::lexer::Keyword::Owner;
use crate::lexer::Keyword::Rename;
use crate::lexer::Keyword::Replica;
use crate::lexer::Keyword::To;
use crate::lexer::Keyword::Trigger;
use crate::parser::ast_node::AlterEventTrigStmt;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterOwnerTarget;
use crate::parser::ast_node::EventTriggerState;
use crate::parser::ast_node::EventTriggerState::Disabled;
use crate::parser::ast_node::EventTriggerState::FiresAlways;
use crate::parser::ast_node::EventTriggerState::FiresOnOrigin;
use crate::parser::ast_node::EventTriggerState::FiresOnReplica;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::RenameTarget;
use crate::parser::col_id;
use crate::parser::combinators::match_first;
use crate::parser::combinators::match_first_with_state;
use crate::parser::combinators::or;
use crate::parser::combinators::sequence;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::role_parsers::role_spec;
