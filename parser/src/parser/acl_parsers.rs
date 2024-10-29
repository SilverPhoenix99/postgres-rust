impl Parser<'_> {

    /// Post-condition: Vec is **Not** empty
    ///
    /// Alias `DefACLOptionList`
    pub(in crate::parser) fn def_acl_option_list(&mut self) -> ScanResult<Vec<AclAction>> {

        let element = self.def_acl_option()?;
        let mut elements = vec![element];

        while let Some(element) = self.def_acl_option().optional()? {
            elements.push(element);
        }

        Ok(elements)
    }

    /// Alias `DefACLOption`
    pub(in crate::parser) fn def_acl_option(&mut self) -> ScanResult<AclAction> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::def_acl_option";

        consume!{self
            Ok {
                Kw(In) => {
                    self.buffer.consume_kw_eq(Schema).required(fn_info!(FN_NAME))?;
                    let schemas = self.name_list().required(fn_info!(FN_NAME))?;
                    Ok(AclAction::Schemas(schemas))
                },
                Kw(For) => {
                    self.buffer.consume_kws(|kw| matches!(kw, Role | User)).required(fn_info!(FN_NAME))?;
                    let roles = self.role_list().required(fn_info!(FN_NAME))?;
                    Ok(AclAction::Roles(roles))
                }
            }
            Err {
                Ok(_) => NoMatch,
                Err(err) => err.into(),
            }
        }
    }

    /// Post-condition: Vec is **Not** empty
    pub(in crate::parser) fn privilege_list(&mut self) -> ScanResult<Vec<AccessPrivilege>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::privilege_list";

        let element = self.privilege()?;
        let mut elements = vec![element];

        while self.buffer.consume_eq(Comma).optional()?.is_some() {
            let element = self.privilege().required(fn_info!(FN_NAME))?;
            elements.push(element);
        }

        Ok(elements)
    }

    fn privilege(&mut self) -> ScanResult<AccessPrivilege> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::privilege";

        let privilege = consume!{self
            Ok {
                Kw(Alter) => {
                    self.buffer.consume_kw_eq(SystemKw).required(fn_info!(FN_NAME))?;
                    Ok(AccessPrivilege::AlterSystem)
                },
                Kw(Create) => {
                    let columns = self.opt_column_list().optional()?;
                    Ok(AccessPrivilege::Create(columns))
                },
                Kw(References) => {
                    let columns = self.opt_column_list().optional()?;
                    Ok(AccessPrivilege::References(columns))
                },
                Kw(Select) => {
                    let columns = self.opt_column_list().optional()?;
                    Ok(AccessPrivilege::Select(columns))
                },
            }
            Err {
                Ok(_) => NoMatch,
                Err(err) => err.into(),
            }
        };

        if let Some(privilege) = privilege.no_match_to_option()? {
            return Ok(privilege);
        }

        let name = self.col_id()?;
        let columns = self.opt_column_list().optional()?;

        Ok(AccessPrivilege::Named(name, columns))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::{AclAction::*, RoleSpec::*};
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_in_schema() {
        let source = "in schema a,b,c";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Schemas(vec!["a".into(), "b".into(), "c".into()])), parser.def_acl_option());
    }

    #[test]
    fn test_for_role() {
        let source = "for role public,current_role";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Roles(vec![Public, CurrentRole])), parser.def_acl_option());
    }

    #[test]
    fn test_for_user() {
        let source = "for user my_user,session_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(Roles(vec![Name("my_user".into()), SessionUser])), parser.def_acl_option());
    }

    #[test]
    fn test_acl_option_list() {
        let source = "in schema my_schema for role public for user current_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = vec![
            Schemas(vec!["my_schema".into()]),
            Roles(vec![Public]),
            Roles(vec![CurrentUser]),
        ];

        assert_eq!(Ok(expected), parser.def_acl_option_list());
    }

    #[test]
    fn test_name_privilege() {
        let source = "some_name another_name(column_name)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = None;
        assert_eq!(Ok(AccessPrivilege::Named("some_name".into(), expected)), parser.privilege());

        let expected = Some(vec!["column_name".into()]);
        assert_eq!(Ok(AccessPrivilege::Named("another_name".into(), expected)), parser.privilege());
    }

    #[test]
    fn test_select_privilege() {
        let source = "select select(column_name)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = None;
        assert_eq!(Ok(AccessPrivilege::Select(expected)), parser.privilege());

        let expected = Some(vec!["column_name".into()]);
        assert_eq!(Ok(AccessPrivilege::Select(expected)), parser.privilege());
    }

    #[test]
    fn test_references_privilege() {
        let source = "references references(column_name)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = None;
        assert_eq!(Ok(AccessPrivilege::References(expected)), parser.privilege());

        let expected = Some(vec!["column_name".into()]);
        assert_eq!(Ok(AccessPrivilege::References(expected)), parser.privilege());
    }

    #[test]
    fn test_create_privilege() {
        let source = "create create(column_name)";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = None;
        assert_eq!(Ok(AccessPrivilege::Create(expected)), parser.privilege());

        let expected = Some(vec!["column_name".into()]);
        assert_eq!(Ok(AccessPrivilege::Create(expected)), parser.privilege());
    }

    #[test]
    fn test_alter_system_privilege() {
        let source = "alter system";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(AccessPrivilege::AlterSystem), parser.privilege());
    }

    #[test]
    fn test_privilege_list() {
        let source = "alter system, select, create, some_privilege";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = vec![
            AccessPrivilege::AlterSystem,
            AccessPrivilege::Select(None),
            AccessPrivilege::Create(None),
            AccessPrivilege::Named("some_privilege".into(), None),
        ];

        assert_eq!(Ok(expected), parser.privilege_list());
    }
}

use crate::lexer::TokenKind::Comma;
use crate::parser::ast_node::AccessPrivilege;
use crate::parser::result::ScanResultTrait;
use crate::{
    lexer::{
        Keyword::{Alter, Create, For, In, References, Role, Schema, Select, SystemKw, User},
        TokenKind::Keyword as Kw
    },
    parser::{
        ast_node::AclAction,
        consume_macro::consume,
        result::{Optional, Required, ScanErrorKind::NoMatch, ScanResult},
        Parser
    }
};
use postgres_basics::fn_info;
