pub(super) fn unicode_normal_form(stream: &mut TokenStream) -> scan::Result<UnicodeNormalForm> {

    alt!(
        Nfc.map(|_| CanonicalComposition),
        Nfd.map(|_| CanonicalDecomposition),
        Nfkc.map(|_| CompatibilityComposition),
        Nfkd.map(|_| CompatibilityDecomposition)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("nfc", CanonicalComposition)]
    #[test_case("nfd", CanonicalDecomposition)]
    #[test_case("nfkc", CompatibilityComposition)]
    #[test_case("nfkd", CompatibilityDecomposition)]
    fn test_unicode_normal_form(source: &str, expected: UnicodeNormalForm) {
        test_parser!(source, unicode_normal_form, expected)
    }
}

use crate::combinators::foundation::alt;
use pg_ast::UnicodeNormalForm;
use pg_ast::UnicodeNormalForm::CanonicalComposition;
use pg_ast::UnicodeNormalForm::CanonicalDecomposition;
use pg_ast::UnicodeNormalForm::CompatibilityComposition;
use pg_ast::UnicodeNormalForm::CompatibilityDecomposition;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Nfc;
use pg_lexer::Keyword::Nfd;
use pg_lexer::Keyword::Nfkc;
use pg_lexer::Keyword::Nfkd;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
