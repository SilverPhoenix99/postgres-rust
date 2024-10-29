mod acl_option_parsers;
mod privilege_parsers;

impl Parser<'_> {

    /// Alias `DefACLAction`
    pub(in crate::parser) fn def_acl_action(&mut self) -> ScanResult<GrantStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::def_acl_action";

        /*
              GRANT privileges ON defacl_privilege_target TO grantee_list opt_grant_option
            | REVOKE ( GRANT OPTION FOR )? privileges ON defacl_privilege_target FROM grantee_list opt_drop_behavior
        */

        consume!{self
            Ok {
                Kw(Grant) => {
                    let privileges = self.privileges().required(fn_info!(FN_NAME))?;
                    self.buffer.consume_kw_eq(On).required(fn_info!(FN_NAME))?;
                    let object_type = self.def_acl_privilege_target().required(fn_info!(FN_NAME))?;
                    self.buffer.consume_kw_eq(To).required(fn_info!(FN_NAME))?;
                    let grantees = self.grantee_list().required(fn_info!(FN_NAME))?;
                    let grant_option = self.opt_grant_option().required(fn_info!(FN_NAME))?;

                    let stmt = GrantStmt::grant(privileges, object_type, grantees, grant_option);
                    Ok(stmt)
                },
                Kw(Revoke) => {
                    let grant_option = if self.buffer.consume_kw_eq(Grant).optional()?.is_some() {
                        /*
                            GRANT OPTION FOR
                        */
                        self.buffer.consume_kw_eq(OptionKw).required(fn_info!(FN_NAME))?;
                        self.buffer.consume_kw_eq(For).required(fn_info!(FN_NAME))?;
                        true
                    }
                    else {
                        false
                    };

                    let privileges = self.privileges().required(fn_info!(FN_NAME))?;
                    self.buffer.consume_kw_eq(On).required(fn_info!(FN_NAME))?;
                    let object_type = self.def_acl_privilege_target().required(fn_info!(FN_NAME))?;
                    self.buffer.consume_kw_eq(FromKw).required(fn_info!(FN_NAME))?;
                    let grantees = self.grantee_list().required(fn_info!(FN_NAME))?;
                    let drop_behavior = self.opt_drop_behavior().required(fn_info!(FN_NAME))?;

                    let stmt = GrantStmt::revoke(privileges, object_type, grantees, grant_option, drop_behavior);
                    Ok(stmt)
                },
            }
            Err {
                Ok(_) => NoMatch,
                Err(err) => err.into(),
            }
        }
    }

    /// Post-condition: Vec is **Not** empty
    pub(in crate::parser) fn grantee_list(&mut self) -> ScanResult<Vec<RoleSpec>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::grantee_list";

        /*
            grantee ( ',' grantee )*
        */

        let element = self.grantee()?;
        let mut elements = vec![element];

        while self.buffer.consume_eq(Comma).optional()?.is_some() {
            let element = self.grantee().required(fn_info!(FN_NAME))?;
            elements.push(element);
        }

        Ok(elements)
    }

    pub(in crate::parser) fn grantee(&mut self) -> ScanResult<RoleSpec> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::grantee";

        /*
              role_spec
            | GROUP role_spec
        */

        let is_required = self.buffer.consume_kw_eq(Group).no_match_to_option()?.is_some();
        let role = self.role_spec();
        if is_required {
            role.required(fn_info!(FN_NAME)).map_err(From::from)
        }
        else {
            role
        }
    }

    /// Alias `opt_grant_grant_option`
    pub(in crate::parser) fn opt_grant_option(&mut self) -> EofResult<bool> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::opt_grant_grant_option";

        /*
            ( WITH GRANT OPTION )?
        */

        if self.buffer.consume_kw_eq(With).optional()?.is_none() {
            return Ok(false)
        }

        self.buffer.consume_kw_eq(Grant).required(fn_info!(FN_NAME))?;
        self.buffer.consume_kw_eq(OptionKw).required(fn_info!(FN_NAME))?;

        Ok(true)
    }

    pub(in crate::parser) fn opt_drop_behavior(&mut self) -> EofResult<DropBehavior> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::opt_drop_behavior";

        /*
            ( CASCADE | RESTRICT )?
        */

        if self.buffer.consume_kw_eq(Cascade).optional()?.is_some() {
            return Ok(DropBehavior::Cascade)
        }

        self.buffer.consume_kw_eq(Restrict).optional()?;

        Ok(DropBehavior::Restrict)
    }

    /// Alias `defacl_privilege_target`
    pub(in crate::parser) fn def_acl_privilege_target(&mut self) -> ScanResult<AclTarget> {

        consume!{self
            Ok {
                Kw(Tables) => Ok(AclTarget::Table),
                Kw(Functions | Routines) => Ok(AclTarget::Function),
                Kw(Sequences) => Ok(AclTarget::Sequence),
                Kw(Types) => Ok(AclTarget::Type),
                Kw(Schemas) => Ok(AclTarget::Schema),
            }
            Err {
                Ok(_) => NoMatch,
                Err(err) => err.into(),
            }
        }
    }

    pub(in crate::parser) fn opt_granted_by(&mut self) -> ScanResult<RoleSpec> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::opt_granted_by";

        /*
            GRANTED BY role_spec
        */

        self.buffer.consume_kw_eq(Granted)?;
        self.buffer.consume_kw_eq(By).required(fn_info!(FN_NAME))?;

        self.role_spec().required(fn_info!(FN_NAME)).map_err(From::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::{AccessPrivilege, RoleSpec};
    use crate::parser::tests::DEFAULT_CONFIG;
    use test_case::test_case;

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

    #[test_case("tables", AclTarget::Table)]
    #[test_case("functions", AclTarget::Function)]
    #[test_case("sequences", AclTarget::Sequence)]
    #[test_case("routines", AclTarget::Function)]
    #[test_case("types", AclTarget::Type)]
    #[test_case("schemas", AclTarget::Schema)]
    fn test_def_acl_privilege_target(source: &str, expected: AclTarget) {
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), parser.def_acl_privilege_target());
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

    #[test]
    fn test_grant_def_acl_action() {
        let source = "grant all privileges on tables to public";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.def_acl_action();

        let expected = GrantStmt::grant(
            AccessPrivilege::All(None),
            AclTarget::Table,
            vec![RoleSpec::Public],
            false
        );

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_grant_with_option_def_acl_action() {
        let source = "grant all privileges on tables to public with grant option";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.def_acl_action();

        let expected = GrantStmt::grant(
            AccessPrivilege::All(None),
            AclTarget::Table,
            vec![RoleSpec::Public],
            true
        );

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_revoke_def_acl_action() {
        let source = "revoke all privileges on tables from public";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.def_acl_action();

        let expected = GrantStmt::revoke(
            AccessPrivilege::All(None),
            AclTarget::Table,
            vec![RoleSpec::Public],
            false,
            DropBehavior::Restrict
        );

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_revoke_grant_option_cascade_def_acl_action() {
        let source = "revoke grant option for all privileges on tables from public cascade";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.def_acl_action();

        let expected = GrantStmt::revoke(
            AccessPrivilege::All(None),
            AclTarget::Table,
            vec![RoleSpec::Public],
            true,
            DropBehavior::Cascade
        );

        assert_eq!(Ok(expected), actual);
    }
}

use crate::{
    lexer::{
        Keyword::{
            By,
            Cascade,
            For,
            FromKw,
            Functions,
            Grant,
            Granted,
            Group,
            On,
            OptionKw,
            Restrict,
            Revoke,
            Routines,
            Schemas,
            Sequences,
            Tables,
            To,
            Types,
            With,
        },
        TokenKind::{
            Comma,
            Keyword as Kw
        }
    },
    parser::{
        ast_node::{AclTarget, DropBehavior, GrantStmt, RoleSpec},
        consume_macro::consume,
        result::{EofResult, Optional, Required, ScanErrorKind::NoMatch, ScanResult, ScanResultTrait},
        Parser
    }
};
use postgres_basics::fn_info;
