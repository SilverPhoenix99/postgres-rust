/// Includes: `AlterEventTrigStmt`
pub(super) fn alter_event_trigger_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER EVENT TRIGGER ColId enable_trigger
        ALTER EVENT TRIGGER ColId OWNER TO RoleSpec
        ALTER EVENT TRIGGER ColId RENAME TO ColId
    */

    sequence!(
        Event.and(Trigger).skip(),
        col_id,
    ).chain(match_first_with_state!(|(_, trigger), stream| {
        { enable_trigger() } => (state) {
            AlterEventTrigStmt::new(trigger, state).into()
        },
        {
            Owner.and(To)
                .and_right(role_spec)
        } => (new_owner) {
            AlterOwnerStmt::new(
                AlterOwnerTarget::EventTrigger(trigger),
                new_owner
            ).into()
        },
        {
            Rename.and(To)
                .and_right(col_id)
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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::RoleSpec;
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

use crate::combinators::col_id;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::match_first_with_state;
use crate::combinators::foundation::or;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_spec;
use pg_ast::AlterEventTrigStmt;
use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::EventTriggerState;
use pg_ast::EventTriggerState::Disabled;
use pg_ast::EventTriggerState::FiresAlways;
use pg_ast::EventTriggerState::FiresOnOrigin;
use pg_ast::EventTriggerState::FiresOnReplica;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_lexer::Keyword::Always;
use pg_lexer::Keyword::Disable;
use pg_lexer::Keyword::Enable;
use pg_lexer::Keyword::Event;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::Replica;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::Trigger;
