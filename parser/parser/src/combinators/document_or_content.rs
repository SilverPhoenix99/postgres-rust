pub(super) fn document_or_content() -> impl Combinator<Output = XmlNodeKind> {

    /*
          DOCUMENT
        | CONTENT
    */

    Kw::Document.map(|_| Document)
        .or(Kw::Content.map(|_| Content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_document_or_content() {
        let mut stream = TokenStream::new("document content", DEFAULT_CONFIG);

        let actual = document_or_content().parse(&mut stream);
        assert_eq!(Ok(Document), actual);

        let actual = document_or_content().parse(&mut stream);
        assert_eq!(Ok(Content), actual);
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::XmlNodeKind;
use postgres_parser_ast::XmlNodeKind::Content;
use postgres_parser_ast::XmlNodeKind::Document;
use postgres_parser_lexer::Keyword as Kw;
