impl Parser<'_> {
    pub(in crate::parser) fn alter_event_trigger_stmt(&mut self) -> OptResult<AstNode> {

        /*
            ALTER EVENT TRIGGER ColId enable_trigger
            ALTER EVENT TRIGGER ColId OWNER TO RoleSpec
            ALTER EVENT TRIGGER ColId RENAME TO ColId
        */

        if self.buffer.consume_kw_eq(Event)?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Trigger).required()?;

        let trigger = self.col_id().required()?;

        let op = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword)
                .filter(|kw| matches!(kw, Owner | Rename))
        ).replace_eof(Err(Some(ParserErrorKind::default())))?;

        let Some(op) = op else {
            let state = self.enable_trigger()?;
            return Ok(Some(
                AlterEventTrigStmt::new(trigger, state).into()
            ))
        };

        self.buffer.consume_kw_eq(To).required()?;

        if op == Owner {
            let new_owner = self.role_spec().required()?;
            Ok(Some(
                AlterOwnerStmt::new(
                    AlterOwnerTarget::EventTrigger(trigger),
                    new_owner
                ).into()
            ))
        }
        else {
            let new_name = self.col_id().required()?;
            Ok(Some(
                RenameStmt::new(
                    RenameTarget::EventTrigger(trigger),
                    new_name
                ).into()
            ))
        }
    }

    fn enable_trigger(&mut self) -> ReqResult<EventTriggerState> {

        /*
            ENABLE_P
          | ENABLE_P REPLICA
          | ENABLE_P ALWAYS
          | DISABLE_P
        */

        let enable = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword)
                .filter(|kw| matches!(kw, Enable | Disable))
                .map(|kw| kw == Enable)
        ).required()?;

        if !enable {
            return Ok(Disabled)
        }

        let enable_option = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword)
                .filter(|kw| matches!(kw, Replica | Always))
        );

        match enable_option {
            Err(Some(err)) => Err(Some(err)),
            Ok(Some(Replica)) => Ok(FiresOnReplica),
            Ok(Some(_)) => Ok(FiresAlways),
            Ok(None) | Err(None) => Ok(FiresOnOrigin),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_alter_enable() {
        let mut parser = Parser::new("event trigger trigger_name enable", DEFAULT_CONFIG);

        let expected = AlterEventTrigStmt::new("trigger_name".into(), FiresOnOrigin);

        assert_eq!(Ok(Some(expected.into())), parser.alter_event_trigger_stmt());
    }

    #[test]
    fn test_alter_owner() {
        let mut parser = Parser::new("event trigger trigger_name owner to public", DEFAULT_CONFIG);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::EventTrigger("trigger_name".into()),
            RoleSpec::Public,
        );

        assert_eq!(Ok(Some(expected.into())), parser.alter_event_trigger_stmt());
    }

    #[test]
    fn test_alter_rename() {
        let mut parser = Parser::new("event trigger trigger_name rename to another_trigger", DEFAULT_CONFIG);

        let expected = RenameStmt::new(
            RenameTarget::EventTrigger("trigger_name".into()),
            "another_trigger".into()
        );

        assert_eq!(Ok(Some(expected.into())), parser.alter_event_trigger_stmt());
    }

    #[test]
    fn test_disable() {
        let mut parser = Parser::new("disable", DEFAULT_CONFIG);
        assert_eq!(Ok(Disabled), parser.enable_trigger());
    }

    #[test]
    fn test_enable() {
        let mut parser = Parser::new("enable", DEFAULT_CONFIG);
        assert_eq!(Ok(FiresOnOrigin), parser.enable_trigger());
    }

    #[test]
    fn test_enable_replica() {
        let mut parser = Parser::new("enable replica", DEFAULT_CONFIG);
        assert_eq!(Ok(FiresOnReplica), parser.enable_trigger());
    }

    #[test]
    fn test_enable_always() {
        let mut parser = Parser::new("enable always", DEFAULT_CONFIG);
        assert_eq!(Ok(FiresAlways), parser.enable_trigger());
    }
}

use crate::lexer::Keyword::{Always, Disable, Enable, Event, Owner, Rename, Replica, To, Trigger};
use crate::lexer::KeywordDetails;
use crate::parser::ast_node::{AlterEventTrigStmt, AlterOwnerStmt, AlterOwnerTarget, RenameStmt, RenameTarget};
use crate::parser::result::OptionalResult;
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::EventTriggerState::{Disabled, FiresAlways, FiresOnOrigin, FiresOnReplica};
use crate::parser::OptResult;
use crate::parser::Parser;
use crate::parser::ParserErrorKind;
use crate::parser::ReqResult;
use crate::parser::{AstNode, EventTriggerState};
