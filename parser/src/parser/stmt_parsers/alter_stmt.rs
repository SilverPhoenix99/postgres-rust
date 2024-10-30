impl Parser<'_> {
    pub(in crate::parser) fn alter_stmt(&mut self) -> ParseResult<RawStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::alter_stmt";

        // ALTER was consumed, so at least one of the following matches is required

        consume! {self FN_NAME
            Ok {
                Kw(Collation) => self.alter_collation_stmt(),
                Kw(Conversion) => self.alter_conversion_stmt(),
                Kw(DefaultKw), Kw(Privileges) => self.alter_default_privileges_stmt().map(From::from),
                Kw(Event) => self.alter_event_trigger_stmt(),
                Kw(Group) => self.alter_group_stmt(),
                Kw(Language) => self.alter_language_stmt(),
                Kw(Procedural), Kw(Language) => self.alter_language_stmt(),
                Kw(Large) => self.alter_large_object_stmt(),
            }
            Err {
                Ok(_) | Err(EofErrorKind::Eof) => syntax_err!(FN_NAME),
                Err(NotEof(err)) => err.clone(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("collation some_name refresh version")]
    #[test_case("conversion some_conversion rename to new_conversion")]
    #[test_case("default privileges in schema some_schema grant all on tables to public")]
    #[test_case("event trigger some_trigger owner to current_user")]
    #[test_case("group some_group rename to new_group_name")]
    #[test_case("language lang owner to session_user")]
    #[test_case("large object -127 owner to public")]
    fn test_alter(source: &str) {

        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.alter_stmt();

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(_) for {source:?} but actually got {actual:?}"
        );
    }
}

use crate::lexer::Keyword::Privileges;
use crate::{
    lexer::{
        Keyword::{Collation, Conversion, DefaultKw, Event, Group, Language, Large, Procedural},
        TokenKind::Keyword as Kw,
    },
    parser::{
        ast_node::RawStmt,
        consume,
        error::syntax_err,
        result::{
            EofErrorKind::{self, NotEof},
            Required
        },
        ParseResult,
        Parser
    }
};
use postgres_basics::fn_info;
