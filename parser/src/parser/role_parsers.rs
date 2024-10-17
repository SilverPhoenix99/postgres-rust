impl Parser<'_> {

    /// Post-condition: Vec is **not** empty
    pub(super) fn role_list(&mut self) -> ScanResult<Vec<RoleSpec>> {

        /*
            role_spec ( ',' role_spec )*
        */

        let role = self.role_spec()?;
        let mut roles = vec![role];

        while self.buffer.consume_eq(Comma).optional()?.is_some() {
            let role = self.role_spec().required()?;
            roles.push(role);
        }

        Ok(roles)
    }

    /// Alias: `RoleId`
    #[inline]
    pub(super) fn role_id(&mut self) -> ScanResult<CowStr> {

        // Similar to role_spec, but only allows an identifier, i.e., disallows builtin roles

        self.role_spec()?
            .into_role_id()
            .map_err(ScanErrorKind::from)
    }

    /// Alias: `RoleSpec`
    pub(super) fn role_spec(&mut self) -> ScanResult<RoleSpec> {

        /*
            role_spec :
                  NONE => Err(ReservedRoleSpec)
                | CURRENT_ROLE
                | CURRENT_USER
                | SESSION_USER
                | "public"
                | non_reserved_word
        */

        if let Some(ident) = self.identifier().no_match_to_option()? {
            return if ident == "public" {
                Ok(RoleSpec::Public)
            }
            else {
                Ok(RoleSpec::Name(ident.into()))
            }
        }

        self.buffer.consume(|tok| {
            use crate::lexer::Keyword::{CurrentRole, CurrentUser, NoneKw, SessionUser};

            let Some(kw) = tok.keyword() else { return Ok(None) };
            let details = kw.details();

            match details.keyword() {
                NoneKw => Err(ReservedRoleSpec("none")),
                CurrentRole => Ok(Some(RoleSpec::CurrentRole)),
                CurrentUser => Ok(Some(RoleSpec::CurrentUser)),
                SessionUser => Ok(Some(RoleSpec::SessionUser)),
                _ => {
                    if details.category() == Reserved {
                        Ok(None)
                    }
                    else {
                        Ok(Some(
                            RoleSpec::Name(details.text().into())
                        ))
                    }
                },
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::result::ScanErrorKind::{NoMatch, ParserErr};
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::ParserErrorKind::ForbiddenRoleSpec;


    #[test]
    fn test_role_list() {
        let source = "public , CuRrEnT_rOlE,CURRENT_USER, session_user ,coalesce,xxYYzz none";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.role_list().unwrap();

        let expected = [
            RoleSpec::Public,
            RoleSpec::CurrentRole,
            RoleSpec::CurrentUser,
            RoleSpec::SessionUser,
            RoleSpec::Name("coalesce".into()),
            RoleSpec::Name("xxyyzz".into()),
        ];

        assert_eq!(expected, actual.as_slice());
    }

    #[test]
    fn test_role_id() {

        let mut parser = Parser::new("coalesce xxyyzz", DEFAULT_CONFIG);
        assert_eq!(Ok("coalesce".into()), parser.role_id());
        assert_eq!(Ok("xxyyzz".into()), parser.role_id());

        let mut parser = Parser::new("none", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ReservedRoleSpec("none"))), parser.role_id());

        let mut parser = Parser::new("public", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ReservedRoleSpec("public"))), parser.role_id());

        let mut parser = Parser::new("current_role", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ForbiddenRoleSpec("CURRENT_ROLE"))), parser.role_id());

        let mut parser = Parser::new("current_user", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ForbiddenRoleSpec("CURRENT_USER"))), parser.role_id());

        let mut parser = Parser::new("session_user", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ForbiddenRoleSpec("SESSION_USER"))), parser.role_id());
    }

    #[test]
    fn test_role_spec() {
        let source = "public CuRrEnT_rOlE CURRENT_USER session_user coalesce xxyyzz";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!(Ok(RoleSpec::Public), parser.role_spec());
        assert_eq!(Ok(RoleSpec::CurrentRole), parser.role_spec());
        assert_eq!(Ok(RoleSpec::CurrentUser), parser.role_spec());
        assert_eq!(Ok(RoleSpec::SessionUser), parser.role_spec());
        assert_eq!(Ok(RoleSpec::Name("coalesce".into())), parser.role_spec());
        assert_eq!(Ok(RoleSpec::Name("xxyyzz".into())), parser.role_spec());

        let mut parser = Parser::new("collate", DEFAULT_CONFIG);
        assert_eq!(Err(NoMatch), parser.role_spec());

        let mut parser = Parser::new("none", DEFAULT_CONFIG);
        assert_eq!(Err(ParserErr(ReservedRoleSpec("none"))), parser.role_spec());
    }
}

use crate::lexer::KeywordCategory::Reserved;
use crate::lexer::TokenKind::Comma;
use crate::parser::ast_node::RoleSpec;
use crate::parser::result::{ScanErrorKind, ScanResult, ScanResultTrait};
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::ParserErrorKind::ReservedRoleSpec;
use crate::parser::{CowStr, Parser};
