enum Change {
    EnableTrigger(EventTriggerState),
    Owner(RoleSpec),
    Name(Str),
}

/// Includes: `AlterEventTrigStmt`
pub(super) fn alter_event_trigger_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        ALTER EVENT TRIGGER ColId enable_trigger
        ALTER EVENT TRIGGER ColId OWNER TO RoleSpec
        ALTER EVENT TRIGGER ColId RENAME TO ColId
    */

    let (.., event_trigger, change) = seq!(Event, Trigger, col_id, changes).parse(stream)?;

    let stmt = match change {
        Change::EnableTrigger(state) => {
            AlterEventTrigStmt::new(event_trigger, state).into()
        },
        Change::Owner(new_owner) => {
            AlterOwnerStmt::new(
                AlterOwnerTarget::EventTrigger(event_trigger),
                new_owner,
            ).into()
        },
        Change::Name(new_name) => {
            RenameStmt::new(
                RenameTarget::EventTrigger(event_trigger),
                new_name,
            ).into()
        },
    };

    Ok(stmt)
}

fn changes(stream: &mut TokenStream) -> scan::Result<Change> {
    alt!(
        enable_trigger.map(Change::EnableTrigger),
        seq!(Owner, To, role_spec)
            .map(|(.., new_owner)| Change::Owner(new_owner)),
        seq!(Rename, To, col_id)
            .map(|(.., new_name)| Change::Name(new_name))
    ).parse(stream)
}

fn enable_trigger(stream: &mut TokenStream) -> scan::Result<EventTriggerState> {

    /*
        ENABLE_P
      | ENABLE_P REPLICA
      | ENABLE_P ALWAYS
      | DISABLE_P
    */

    alt!(
        Disable.map(|_| Disabled),
        seq!(
            Enable,
            alt!(
                Replica.map(|_| FiresOnReplica),
                Always.map(|_| FiresAlways)
            )
            .optional()
        )
            .map(|(_, enable)| enable.unwrap_or(FiresOnOrigin))
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::RoleSpec;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case(
        "event trigger trigger_name enable",
        AlterEventTrigStmt::new("trigger_name", FiresOnOrigin).into()
    )]
    #[test_case(
        "event trigger trigger_name owner to public",
        AlterOwnerStmt::new(
            AlterOwnerTarget::EventTrigger("trigger_name".into()),
            RoleSpec::Public,
        ).into()
    )]
    #[test_case(
        "event trigger trigger_name rename to another_trigger",
        RenameStmt::new(
            RenameTarget::EventTrigger("trigger_name".into()),
            "another_trigger"
        ).into()
    )]
    fn test_alter_enable(source: &str, expected: RawStmt) {
        test_parser!(source, alter_event_trigger_stmt, expected)
    }

    #[test_case("disable", Disabled)]
    #[test_case("enable", FiresOnOrigin)]
    #[test_case("enable replica", FiresOnReplica)]
    #[test_case("enable always", FiresAlways)]
    fn test_enable_trigger(source: &str, expected: EventTriggerState) {
        test_parser!(source, enable_trigger, expected)
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::alt;
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
use pg_ast::RoleSpec;
use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Always;
use pg_lexer::Keyword::Disable;
use pg_lexer::Keyword::Enable;
use pg_lexer::Keyword::Event;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::Replica;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::Trigger;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
