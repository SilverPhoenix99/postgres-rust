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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_document_or_content() {
        let mut stream = TokenStream::new("document content", DEFAULT_CONFIG);

        let actual = document_or_content(&mut stream);
        assert_eq!(Ok(Document), actual);

        let actual = document_or_content(&mut stream);
        assert_eq!(Ok(Content), actual);
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::XmlNodeKind;
use pg_ast::XmlNodeKind::Content;
use pg_ast::XmlNodeKind::Document;
use pg_lexer::Keyword as Kw;
