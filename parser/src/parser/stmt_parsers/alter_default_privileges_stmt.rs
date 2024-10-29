impl Parser<'_> {

    /// Post-condition: Vec is **Not** empty
    ///
    /// Alias `DefACLOptionList`
    fn def_acl_option_list(&mut self) -> ScanResult<Vec<AclOption>> {

        let element = self.def_acl_option()?;
        let mut elements = vec![element];

        while let Some(element) = self.def_acl_option().optional()? {
            elements.push(element);
        }

        Ok(elements)
    }

    /// Alias `DefACLOption`
    fn def_acl_option(&mut self) -> ScanResult<AclOption> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::def_acl_option";

        consume!{self
            Ok {
                Kw(In) => {
                    self.buffer.consume_kw_eq(Schema).required(fn_info!(FN_NAME))?;
                    let schemas = self.name_list().required(fn_info!(FN_NAME))?;
                    Ok(AclOption::Schemas(schemas))
                },
                Kw(For) => {
                    self.buffer.consume_kws(|kw| matches!(kw, Role | User)).required(fn_info!(FN_NAME))?;
                    let roles = self.role_list().required(fn_info!(FN_NAME))?;
                    Ok(AclOption::Roles(roles))
                }
            }
            Err {
                Ok(_) => NoMatch,
                Err(err) => err.into(),
            }
        }
    }

    /// Alias `DefACLAction`
    fn def_acl_action(&mut self) -> ScanResult<GrantStmt> {
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

    /// Alias `defacl_privilege_target`
    fn def_acl_privilege_target(&mut self) -> ScanResult<AclTarget> {

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::{AccessPrivilege, AclOption::*, AclTarget, DropBehavior, RoleSpec::*};
    use crate::parser::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test]
    fn test_acl_option_list() {
        let source = "in schema my_schema for role public for user current_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = vec![
            AclOption::Schemas(vec!["my_schema".into()]),
            Roles(vec![Public]),
            Roles(vec![CurrentUser]),
        ];

        assert_eq!(Ok(expected), parser.def_acl_option_list());
    }

    #[test]
    fn test_def_acl_option_in_schema() {
        let source = "in schema a,b,c";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(AclOption::Schemas(vec!["a".into(), "b".into(), "c".into()])), parser.def_acl_option());
    }

    #[test]
    fn test_def_acl_option_for_role() {
        let source = "for role public,current_role";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Roles(vec![Public, CurrentRole])), parser.def_acl_option());
    }

    #[test]
    fn test_def_acl_option_for_user() {
        let source = "for user my_user,session_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Roles(vec![Name("my_user".into()), SessionUser])), parser.def_acl_option());
    }

    #[test]
    fn test_grant_def_acl_action() {
        let source = "grant all privileges on tables to public";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.def_acl_action();

        let expected = GrantStmt::grant(
            AccessPrivilege::All(None),
            AclTarget::Table,
            vec![Public],
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
            vec![Public],
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
            vec![Public],
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
            vec![Public],
            true,
            DropBehavior::Cascade
        );

        assert_eq!(Ok(expected), actual);
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
}

use crate::lexer::Keyword::{
    For,
    FromKw,
    Functions,
    Grant,
    In,
    On,
    OptionKw,
    Revoke,
    Role,
    Routines,
    Schema,
    Schemas,
    Sequences,
    Tables,
    To,
    Types,
    User,
};
use crate::lexer::TokenKind::Keyword as Kw;
use crate::parser::ast_node::{AclOption, AclTarget, GrantStmt};
use crate::parser::consume_macro::consume;
use crate::parser::result::ScanErrorKind::NoMatch;
use crate::parser::result::{Optional, Required, ScanResult};
use crate::parser::Parser;
use postgres_basics::fn_info;
