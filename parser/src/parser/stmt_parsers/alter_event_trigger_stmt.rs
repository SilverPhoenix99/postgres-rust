impl Parser<'_> {
    pub(in crate::parser) fn alter_event_trigger_stmt(&mut self) -> OptResult<AstNode> {

        /*
            ALTER EVENT TRIGGER ColId enable_trigger    # AlterEventTrigStmt
          | ALTER EVENT TRIGGER ColId OWNER TO RoleSpec # AlterOwnerStmt
          | ALTER EVENT TRIGGER ColId RENAME TO ColId   # RenameStmt
        */

        if self.buffer.consume_kw_eq(Unreserved(Event))?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Unreserved(Trigger)).required()?;

        let trigger = self.col_id().required()?;

        let op = self.buffer.consume(|tok|
            tok.keyword().and_then(KeywordDetails::unreserved)
                .filter(|kw| matches!(kw, Owner | Rename))
        ).replace_eof(Err(Some(ParserErrorKind::default())))?;

        if op.is_none() {
            let state = self.enable_trigger()?;
            return Ok(Some(AlterEventTrigStmt { trigger, state }))
        }
        // SAFETY: checked in the condition above
        let op = op.unwrap();

        self.buffer.consume_kw_eq(Reserved(To)).required()?;

        if op == Owner {
            let new_owner = self.role_spec().required()?;
            Ok(Some(
                AlterOwnerStmt::EventTrigger { trigger, new_owner }.into()
            ))
        }
        else {
            let new_name = self.col_id().required()?;
            Ok(Some(
                RenameStmt::EventTrigger { trigger, new_name }.into()
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
            tok.keyword().and_then(KeywordDetails::unreserved)
                .filter(|kw| matches!(kw, Enable | Disable))
                .map(|kw| kw == Enable)
        ).required()?;

        if !enable {
            return Ok(Disabled)
        }

        let enable_option = self.buffer.consume(|tok|
            tok.keyword().and_then(KeywordDetails::unreserved)
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
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::EventTriggerState::FiresAlways;
    use crate::parser::RoleSpec;

    #[test]
    fn test_alter_enable() {
        let mut parser = Parser::new(b"event trigger trigger_name enable", DEFAULT_CONFIG);

        let expected = AlterEventTrigStmt {
            trigger: "trigger_name".into(),
            state: FiresOnOrigin,
        };

        assert_eq!(Ok(Some(expected)), parser.alter_event_trigger_stmt());
    }

    #[test]
    fn test_alter_owner() {
        let mut parser = Parser::new(b"event trigger trigger_name owner to public", DEFAULT_CONFIG);

        let expected = AlterOwnerStmt::EventTrigger {
            trigger: "trigger_name".into(),
            new_owner: RoleSpec::Public,
        };

        assert_eq!(Ok(Some(expected.into())), parser.alter_event_trigger_stmt());
    }

    #[test]
    fn test_alter_rename() {
        let mut parser = Parser::new(b"event trigger trigger_name rename to another_trigger", DEFAULT_CONFIG);

        let expected = RenameStmt::EventTrigger {
            trigger: "trigger_name".into(),
            new_name: "another_trigger".into(),
        };

        assert_eq!(Ok(Some(expected.into())), parser.alter_event_trigger_stmt());
    }

    #[test]
    fn test_disable() {
        let mut parser = Parser::new(b"disable", DEFAULT_CONFIG);
        assert_eq!(Ok(Disabled), parser.enable_trigger());
    }

    #[test]
    fn test_enable() {
        let mut parser = Parser::new(b"enable", DEFAULT_CONFIG);
        assert_eq!(Ok(FiresOnOrigin), parser.enable_trigger());
    }

    #[test]
    fn test_enable_replica() {
        let mut parser = Parser::new(b"enable replica", DEFAULT_CONFIG);
        assert_eq!(Ok(FiresOnReplica), parser.enable_trigger());
    }

    #[test]
    fn test_enable_always() {
        let mut parser = Parser::new(b"enable always", DEFAULT_CONFIG);
        assert_eq!(Ok(FiresAlways), parser.enable_trigger());
    }
}

use crate::lexer::Keyword::{Reserved, Unreserved};
use crate::lexer::ReservedKeyword::To;
use crate::lexer::UnreservedKeyword::{Event, Trigger};
use crate::lexer::{KeywordDetails, UnreservedKeyword, UnreservedKeyword::{Always, Disable, Enable, Replica}};
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::AstNode::AlterEventTrigStmt;
use crate::parser::{result::OptionalResult, token_buffer::TokenConsumer, AstNode, EventTriggerState, EventTriggerState::{Disabled, FiresAlways, FiresOnOrigin, FiresOnReplica}, OptResult, Parser, ParserErrorKind, RenameStmt, ReqResult};
use UnreservedKeyword::{Owner, Rename};
