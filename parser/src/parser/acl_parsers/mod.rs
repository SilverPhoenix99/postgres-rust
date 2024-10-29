mod acl_option_parsers;
mod privilege_parsers;

impl Parser<'_> {

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
    use crate::parser::tests::DEFAULT_CONFIG;
    use test_case::test_case;

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

use crate::parser::ast_node::AclTarget;
use crate::{
    lexer::{
        Keyword::{
            Functions,
            Routines,
            Schemas,
            Sequences,
            Tables,
            Types,
        },
        TokenKind::Keyword as Kw
    },
    parser::{
        consume_macro::consume,
        result::{ScanErrorKind::NoMatch, ScanResult},
        Parser
    }
};
