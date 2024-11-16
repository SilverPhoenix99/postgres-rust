impl Parser<'_> {

    /// Post-condition: Vec is **Not** empty
    pub(super) fn grantee_list(&mut self) -> ScanResult<Vec<RoleSpec>> {

        /*
            grantee ( ',' grantee )*
        */

        let element = self.grantee()?;
        let mut elements = vec![element];

        while self.buffer.consume_op(Comma).optional()?.is_some() {
            let element = self.grantee().required(fn_info!())?;
            elements.push(element);
        }

        Ok(elements)
    }

    fn grantee(&mut self) -> ScanResult<RoleSpec> {

        /*
              role_spec
            | GROUP role_spec
        */

        let is_required = self.buffer.consume_kw_eq(Group).no_match_to_option()?.is_some();
        let role = self.role_spec();
        if is_required {
            role.required(fn_info!()).map_err(From::from)
        }
        else {
            role
        }
    }

    /// Alias: `opt_grant_grant_option`
    pub(super) fn opt_grant_option(&mut self) -> EofResult<bool> {

        /*
            ( WITH GRANT OPTION )?
        */

        if self.buffer.consume_kw_eq(With).optional()?.is_none() {
            return Ok(false)
        }

        self.buffer.consume_kw_eq(Grant).required(fn_info!())?;
        self.buffer.consume_kw_eq(OptionKw).required(fn_info!())?;

        Ok(true)
    }

    pub(super) fn opt_drop_behavior(&mut self) -> EofResult<DropBehavior> {

        /*
            ( CASCADE | RESTRICT )?
        */

        if self.buffer.consume_kw_eq(Cascade).optional()?.is_some() {
            return Ok(DropBehavior::Cascade)
        }

        self.buffer.consume_kw_eq(Restrict).optional()?;

        Ok(DropBehavior::Restrict)
    }

    pub(super) fn opt_granted_by(&mut self) -> ScanResult<RoleSpec> {

        /*
            GRANTED BY role_spec
        */

        self.buffer.consume_kw_eq(Granted)?;
        self.buffer.consume_kw_eq(By).required(fn_info!())?;

        self.role_spec().required(fn_info!()).map_err(From::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_grantee_list() {
        let source = "group session_user, current_role";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = vec![
            RoleSpec::SessionUser,
            RoleSpec::CurrentRole
        ];

        assert_eq!(Ok(expected), parser.grantee_list());
    }

    #[test]
    fn test_grantee() {
        let source = "current_user group public";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(RoleSpec::CurrentUser), parser.grantee());
        assert_eq!(Ok(RoleSpec::Public), parser.grantee());
    }

    #[test]
    fn test_opt_grant_option() {
        let mut parser = Parser::new("with grant option", DEFAULT_CONFIG);
        assert_eq!(Ok(true), parser.opt_grant_option());
        assert_eq!(Ok(false), parser.opt_grant_option());
    }

    #[test]
    fn test_opt_drop_behavior() {
        let mut parser = Parser::new("restrict cascade", DEFAULT_CONFIG);
        assert_eq!(Ok(DropBehavior::Restrict), parser.opt_drop_behavior());
        assert_eq!(Ok(DropBehavior::Cascade), parser.opt_drop_behavior());
        assert_eq!(Ok(DropBehavior::Restrict), parser.opt_drop_behavior());
    }

    #[test]
    fn test_opt_granted_by() {
        let mut parser = Parser::new("granted by public", DEFAULT_CONFIG);
        assert_eq!(Ok(RoleSpec::Public), parser.opt_granted_by());
    }
}

use crate::{
    lexer::{
        Keyword::{By, Cascade, Grant, Granted, Group, OptionKw, Restrict, With},
        OperatorKind::Comma
    },
    parser::{
        ast_node::{DropBehavior, RoleSpec},
        result::{EofResult, Optional, Required, ScanResult, ScanResultTrait},
        Parser
    }
};
use postgres_basics::fn_info;
