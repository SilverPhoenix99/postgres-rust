pub(super) fn document_or_content(stream: &mut TokenStream) -> scan::Result<XmlNodeKind> {

    /*
          DOCUMENT
        | CONTENT
    */

    alt!(
        Kw::Document.map(|_| Document),
        Kw::Content.map(|_| Content)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_or_content() {
        let mut stream = TokenStream::from("document content");

        let actual = document_or_content(&mut stream);
        assert_eq!(Ok(Document), actual);

        let actual = document_or_content(&mut stream);
        assert_eq!(Ok(Content), actual);
    }
}

use crate::combinators::foundation::alt;
use pg_ast::XmlNodeKind;
use pg_ast::XmlNodeKind::Content;
use pg_ast::XmlNodeKind::Document;
use pg_combinators::Combinator;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
