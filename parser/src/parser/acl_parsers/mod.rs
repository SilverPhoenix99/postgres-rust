mod acl_option_parsers;
mod privilege_parsers;

impl Parser<'_> {

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec;
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
}

use crate::{
    lexer::{
        Keyword::{
            Functions,
            Group,
            Routines,
            Schemas,
            Sequences,
            Tables,
            Types,
        },
        TokenKind::{
            Comma,
            Keyword as Kw
        }
    },
    parser::{
        ast_node::{AclTarget, RoleSpec},
        consume_macro::consume,
        result::{Optional, Required, ScanErrorKind::NoMatch, ScanResult, ScanResultTrait},
        Parser
    }
};
use postgres_basics::fn_info;
