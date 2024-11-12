impl Parser<'_> {
    pub(in crate::parser) fn alter_event_trigger_stmt(&mut self) -> ParseResult<RawStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::alter_event_trigger_stmt";

        /*
            ALTER EVENT TRIGGER ColId enable_trigger
            ALTER EVENT TRIGGER ColId OWNER TO RoleSpec
            ALTER EVENT TRIGGER ColId RENAME TO ColId
        */

        self.buffer.consume_kw_eq(Trigger).required(fn_info!(FN_NAME))?;

        let trigger = self.col_id().required(fn_info!(FN_NAME))?;

        let op = self.buffer.consume_kw(|kw| matches!(kw, Owner | Rename))
            .try_match(fn_info!(FN_NAME))?;

        let Some(op) = op else {
            /*
                ... enable_trigger
            */
            let state = self.enable_trigger()?;
            let stmt = AlterEventTrigStmt::new(trigger, state);
            return Ok(stmt.into())
        };

        self.buffer.consume_kw_eq(To).required(fn_info!(FN_NAME))?;

        let stmt = if op == Owner {
            /*
                ... OWNER TO RoleSpec
            */
            let new_owner = self.role_spec().required(fn_info!(FN_NAME))?;
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
            let new_name = self.col_id().required(fn_info!(FN_NAME))?;
            let stmt = RenameStmt::new(
                RenameTarget::EventTrigger(trigger),
                new_name
            );
            stmt.into()
        };

        Ok(stmt)
    }

    fn enable_trigger(&mut self) -> ParseResult<EventTriggerState> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::enable_trigger";

        /*
            ENABLE_P
          | ENABLE_P REPLICA
          | ENABLE_P ALWAYS
          | DISABLE_P
        */

        let enable = self.buffer
            .consume(|tok|
                tok.keyword()
                    .filter(|kw| matches!(kw, Enable | Disable))
                    .map(|kw| kw == Enable)
            )
            .required(fn_info!(FN_NAME))?;

        if !enable {
            return Ok(Disabled)
        }

        let enable_option = self.buffer
            .consume(|tok| match tok.keyword()? {
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
        let mut parser = Parser::new("trigger trigger_name enable", DEFAULT_CONFIG);

        let expected = AlterEventTrigStmt::new("trigger_name".into(), FiresOnOrigin);

        assert_eq!(Ok(expected.into()), parser.alter_event_trigger_stmt());
    }

    #[test]
    fn test_alter_owner() {
        let mut parser = Parser::new("trigger trigger_name owner to public", DEFAULT_CONFIG);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::EventTrigger("trigger_name".into()),
            RoleSpec::Public,
        );

        assert_eq!(Ok(expected.into()), parser.alter_event_trigger_stmt());
    }

    #[test]
    fn test_alter_rename() {
        let mut parser = Parser::new("trigger trigger_name rename to another_trigger", DEFAULT_CONFIG);

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

use crate::{
    lexer::Keyword::{Always, Disable, Enable, Owner, Rename, Replica, To, Trigger},
    parser::{
        ast_node::{AlterEventTrigStmt, AlterOwnerStmt, AlterOwnerTarget, RawStmt, RenameStmt, RenameTarget},
        result::{Optional, Required, TryMatch},
        token_buffer::TokenConsumer,
        EventTriggerState::{self, Disabled, FiresAlways, FiresOnOrigin, FiresOnReplica},
        ParseResult,
        Parser
    },
};
use postgres_basics::fn_info;
