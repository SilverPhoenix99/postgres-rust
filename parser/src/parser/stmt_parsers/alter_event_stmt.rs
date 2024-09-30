impl Parser<'_> {
    fn alter_event_stmt(&mut self) -> OptResult<AstNode> {

        /*
            ALTER EVENT TRIGGER ColId enable_trigger    # AlterEventTrigStmt
          | ALTER EVENT TRIGGER ColId OWNER TO RoleSpec # AlterOwnerStmt
          | ALTER EVENT TRIGGER ColId RENAME TO ColId   # RenameStmt
        */

        todo!()
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

use crate::lexer::{
    KeywordDetails,
    UnreservedKeyword::{Always, Disable, Enable, Replica},
};
use crate::parser::{
    result::OptionalResult,
    token_buffer::TokenConsumer,
    AstNode,
    EventTriggerState,
    EventTriggerState::{Disabled, FiresAlways, FiresOnOrigin, FiresOnReplica},
    OptResult,
    Parser,
    ReqResult,
};
