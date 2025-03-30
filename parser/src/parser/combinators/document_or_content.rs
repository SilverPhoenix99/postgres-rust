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
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_document_or_content() {
        let mut stream = TokenStream::new("document content", DEFAULT_CONFIG);

        let actual = document_or_content().parse(&mut stream);
        assert_eq!(Ok(Document), actual);

        let actual = document_or_content().parse(&mut stream);
        assert_eq!(Ok(Content), actual);
    }
}

use crate::lexer::Keyword as Kw;
use crate::parser::ast_node::XmlNodeKind;
use crate::parser::ast_node::XmlNodeKind::Content;
use crate::parser::ast_node::XmlNodeKind::Document;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
