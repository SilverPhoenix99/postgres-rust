impl Parser<'_> {
    pub(in crate::parser) fn alter_stmt(&mut self) -> ParseResult<RawStmt> {
        use TokenKind::Keyword as Kw;

        // ALTER was consumed, so at least one of the following matches is required

        // alternatives!(
        //     alter_large_object_stmt,
        // );

        consume! {self default,
            Kw(Group) => self.alter_group_stmt(),
            Kw(Event) => self.alter_event_trigger_stmt(),
            Kw(Collation) => self.alter_collation_stmt(),
            Kw(Conversion) => self.alter_conversion_stmt(),
            Kw(Language) => self.alter_language_stmt(),
            Kw(Procedural) => {
                self.buffer.consume_kw_eq(Language).required()?;
                self.alter_language_stmt()
            },
            Kw(Large) => self.alter_large_object_stmt(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_alter() {
        let sources = [
            "group some_group rename to new_group_name",
            "event trigger some_trigger owner to current_user",
            "collation some_name refresh version",
            "conversion some_conversion rename to new_conversion",
            "language lang owner to session_user",
            "large object -127 owner to public",
        ];

        for source in sources {
            let mut parser = Parser::new(source, DEFAULT_CONFIG);
            let actual = parser.alter_stmt();

            // This only quickly tests that statement types aren't missing.
            // More in-depth testing is within each statement's module.
            assert_matches!(actual, Ok(_),
                r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
            );
        }
    }
}

use crate::lexer::Keyword::{Group, Large};
use crate::lexer::{Keyword, TokenKind};
use crate::parser::ast_node::RawStmt;
use crate::parser::result::ScanResultTrait;
use crate::parser::{consume, ParseResult, Parser};
use Keyword::{Collation, Conversion, Event, Language, Procedural};
