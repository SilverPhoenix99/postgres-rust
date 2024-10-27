impl Parser<'_> {

    /// Post-condition: Vec is **not** empty
    pub(super) fn role_list(&mut self) -> ScanResult<Vec<RoleSpec>> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::role_list";

        /*
            role_spec ( ',' role_spec )*
        */

        let role = self.role_spec()?;
        let mut roles = vec![role];

        while self.buffer.consume_eq(Comma).optional()?.is_some() {
            let role = self.role_spec().required(fn_info!(FN_NAME))?;
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
            .map_err(ScanErr)
    }

    /// Alias: `RoleSpec`
    pub(super) fn role_spec(&mut self) -> ScanResult<RoleSpec> {
        use crate::lexer::TokenKind::Keyword as Kw;
        const FN_NAME: &str = "postgres_parser::parser::Parser::role_spec";

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

        consume! {self
            ok {
                Ok(Kw(CurrentRole)) => Ok(RoleSpec::CurrentRole),
                Ok(Kw(CurrentUser)) => Ok(RoleSpec::CurrentUser),
                Ok(Kw(SessionUser)) => Ok(RoleSpec::SessionUser),
                Ok(Kw(kw)) if kw != NoneKw && kw.details().category() != Reserved => {
                    Ok(RoleSpec::Name(kw.details().text().into()))
                }
            }
            err {
                Ok(Kw(NoneKw)) => Err(ScanErr(
                    PartialParserError::new(ReservedRoleSpec("none"), fn_info!(FN_NAME))
                )),
                Ok(_) => Err(NoMatch),
                Err(e) => Err(e.into()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::ParserErrorKind;
    use crate::parser::ParserErrorKind::ForbiddenRoleSpec;
    use std::fmt::Debug;

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
        assert_err(ReservedRoleSpec("none"), parser.role_id());

        let mut parser = Parser::new("public", DEFAULT_CONFIG);
        assert_err(ReservedRoleSpec("public"), parser.role_id());

        let mut parser = Parser::new("current_role", DEFAULT_CONFIG);
        assert_err(ForbiddenRoleSpec("CURRENT_ROLE"), parser.role_id());

        let mut parser = Parser::new("current_user", DEFAULT_CONFIG);
        assert_err(ForbiddenRoleSpec("CURRENT_USER"), parser.role_id());

        let mut parser = Parser::new("session_user", DEFAULT_CONFIG);
        assert_err(ForbiddenRoleSpec("SESSION_USER"), parser.role_id());
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
        assert_err(ReservedRoleSpec("none"), parser.role_spec());
    }

    fn assert_err<T: Debug>(expected: ParserErrorKind, actual: ScanResult<T>) {
        assert_matches!(actual, Err(ScanErr(_)));
        let ScanErr(actual) = actual.unwrap_err() else {
            unreachable!("already checked for Err(ScanErr(_))");
        };

        assert_eq!(&expected, actual.source());
    }
}

use crate::parser::error::PartialParserError;
use crate::{
    lexer::{
        Keyword::{CurrentRole, CurrentUser, NoneKw, SessionUser},
        KeywordCategory::Reserved,
        TokenKind::Comma
    },
    parser::{
        ast_node::RoleSpec,
        consume_macro::consume,
        result::{
            Optional,
            Required,
            ScanErrorKind::{NoMatch, ScanErr},
            ScanResult,
            ScanResultTrait,
        },
        CowStr,
        Parser,
        ParserErrorKind::ReservedRoleSpec,
    },
};
use postgres_basics::fn_info;
