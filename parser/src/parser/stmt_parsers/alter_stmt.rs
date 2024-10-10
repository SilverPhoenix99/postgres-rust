impl Parser<'_> {
    pub(in crate::parser) fn alter_stmt(&mut self) -> ScanResult<AstNode> {

        macro_rules! alternatives {
            ($($func:ident),* $(,)?) => {

                if !self.buffer.eof() {
                    $(if let Some(node) = self.$func().optional()? {
                        return Ok(node)
                    })*
                }

                return Err(ScanErrorKind::default())
            }
        }

        self.buffer.consume_kw_eq(Alter)?;

        // ALTER was consumed, so at least one of the following matches is required

        alternatives!(
            alter_group_stmt,
            alter_event_trigger_stmt,
            alter_collation_stmt,
            alter_conversion_stmt,
            alter_language_stmt,
            alter_large_object_stmt,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_alter() {
        let sources = [
            "alter group some_group rename to new_group_name",
            "alter event trigger some_trigger owner to current_user",
            "alter collation some_name refresh version",
            "alter conversion some_conversion rename to new_conversion",
            "alter language lang owner to session_user",
            "alter large object -127 owner to public",
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

use crate::lexer::Keyword::Alter;
use crate::parser::result::{ScanErrorKind, ScanResult};
use crate::parser::{AstNode, Parser, ScanResultTrait};
