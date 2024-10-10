impl Parser<'_> {
    pub(in crate::parser) fn alter_event_trigger_stmt(&mut self) -> Result<AstNode, ScanErrorKind> {

        /*
            ALTER EVENT TRIGGER ColId enable_trigger
            ALTER EVENT TRIGGER ColId OWNER TO RoleSpec
            ALTER EVENT TRIGGER ColId RENAME TO ColId
        */

        self.buffer.consume_kw_eq(Event)?;
        self.buffer.consume_kw_eq(Trigger).required()?;

        let trigger = self.col_id().required()?;

        let op = self.buffer
            .consume(|tok|
                tok.keyword().map(KeywordDetails::keyword)
                    .filter(|kw| matches!(kw, Owner | Rename))
            )
            .no_match_to_option()
            .required()?;

        let Some(op) = op else {
            /*
                ... enable_trigger
            */
            let state = self.enable_trigger()?;
            let stmt = AlterEventTrigStmt::new(trigger, state);
            return Ok(stmt.into())
        };

        self.buffer.consume_kw_eq(To).required()?;

        let stmt = if op == Owner {
            /*
                ... OWNER TO RoleSpec
            */
            let new_owner = self.role_spec().required()?;
            let stmt = AlterOwnerStmt::new(
                AlterOwnerTarget::EventTrigger(trigger),
                new_owner
            );
            stmt.into()
        }
        else {
            /*
                ... RENAME TO ColId
            */
            let new_name = self.col_id().required()?;
            let stmt = RenameStmt::new(
                RenameTarget::EventTrigger(trigger),
                new_name
            );
            stmt.into()
        };

        Ok(stmt)
    }

    fn enable_trigger(&mut self) -> Result<EventTriggerState, ParserErrorKind> {

        /*
            ENABLE_P
          | ENABLE_P REPLICA
          | ENABLE_P ALWAYS
          | DISABLE_P
        */

        let enable = self.buffer
            .consume(|tok|
                tok.keyword().map(KeywordDetails::keyword)
                    .filter(|kw| matches!(kw, Enable | Disable))
                    .map(|kw| kw == Enable)
            )
            .required()?;

        if !enable {
            return Ok(Disabled)
        }

        let enable_option = self.buffer
            .consume(|tok| match tok.keyword().map(KeywordDetails::keyword)? {
                Replica => Some(FiresOnReplica),
                Always => Some(FiresAlways),
                _ => None,
            })
            .optional()?
            .unwrap_or(FiresOnOrigin);

        Ok(enable_option)
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

        assert_eq!(Ok(expected.into()), parser.alter_event_trigger_stmt());
    }

    #[test]
    fn test_alter_owner() {
        let mut parser = Parser::new("event trigger trigger_name owner to public", DEFAULT_CONFIG);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::EventTrigger("trigger_name".into()),
            RoleSpec::Public,
        );

        assert_eq!(Ok(expected.into()), parser.alter_event_trigger_stmt());
    }

    #[test]
    fn test_alter_rename() {
        let mut parser = Parser::new("event trigger trigger_name rename to another_trigger", DEFAULT_CONFIG);

        let expected = RenameStmt::new(
            RenameTarget::EventTrigger("trigger_name".into()),
            "another_trigger".into()
        );

        assert_eq!(Ok(expected.into()), parser.alter_event_trigger_stmt());
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
use crate::parser::result::{EofResult, ScanErrorKind, ScanResult};
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::EventTriggerState::{Disabled, FiresAlways, FiresOnOrigin, FiresOnReplica};
use crate::parser::{AstNode, EventTriggerState, Parser, ParserErrorKind};
