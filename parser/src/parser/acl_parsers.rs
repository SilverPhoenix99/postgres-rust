impl Parser<'_> {

    /// Post-condition: Vec is **not** empty
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::AclAction::*;
    use crate::parser::ast_node::RoleSpec::*;
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
}

use crate::{
    lexer::{
        Keyword::{For, In, Role, Schema, User},
        TokenKind::Keyword as Kw
    },
    parser::{
        ast_node::AclAction,
        consume_macro::consume,
        result::{Optional, Required, ScanErrorKind::NoMatch, ScanResult},
        CowStr,
        Parser
    }
};
use postgres_basics::fn_info;
