pub(super) fn document_or_content(ctx: &mut ParserContext) -> scan::Result<XmlNodeKind> {

    /*
          DOCUMENT
        | CONTENT
    */

    alt!(
        Kw::Document.map(|_| Document),
        Kw::Content.map(|_| Content)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_or_content() {
        let mut ctx = ParserContext::from("document content");

        let actual = document_or_content(&mut ctx);
        assert_eq!(Ok(Document), actual);

        let actual = document_or_content(&mut ctx);
        assert_eq!(Ok(Content), actual);
    }
}

use pg_ast::XmlNodeKind;
use pg_ast::XmlNodeKind::Content;
use pg_ast::XmlNodeKind::Document;
use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
