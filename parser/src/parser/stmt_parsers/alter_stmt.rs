impl Parser<'_> {
    pub(in crate::parser) fn alter_stmt(&mut self) -> OptResult<AstNode> {

        if self.buffer.consume_kw_eq(Unreserved(Alter))?.is_none() {
            return Ok(None)
        }

        if let Some(node) = self.alter_group_stmt()? { return Ok(Some(node)) }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_alter_group() {
        let sources = [
            "alter group some_group rename to new_group_name"
        ];

        for source in sources {
            let mut parser = Parser::new(source.as_bytes(), DEFAULT_CONFIG);
            let actual = parser.alter_stmt();

            // This only quickly tests that statement types aren't missing.
            // More in-depth testing is within each statement's module.
            assert_matches!(actual, Ok(Some(_)),
                r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
            );
        }
    }
}

use crate::lexer::Keyword::Unreserved;
use crate::lexer::UnreservedKeyword::Alter;
use crate::parser::{AstNode, OptResult, Parser};
