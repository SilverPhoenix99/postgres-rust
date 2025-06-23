#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum TextSearch {
    Configuration(QualifiedName),
    Dictionary(QualifiedName),
    Parser(QualifiedName),
    Template(QualifiedName),
}

pub(super) fn text_search(stream: &mut TokenStream) -> Result<TextSearch> {

    /*
        TEXT SEARCH (
              CONFIGURATION
            | DICTIONARY
            | PARSER
            | TEMPLATE
         ) any_name
    */

    seq!(=>
        Text.parse(stream),
        Search.parse(stream),
        choice!(stream =>
            seq!(stream => Configuration, any_name)
                .map(|(_, name)| TextSearch::Configuration(name)),
            seq!(stream => Dictionary, any_name)
                .map(|(_, name)| TextSearch::Dictionary(name)),
            seq!(stream => ParserKw, any_name)
                .map(|(_, name)| TextSearch::Parser(name)),
            seq!(stream => Template, any_name)
                .map(|(_, name)| TextSearch::Template(name))
        )
    )
        .map(|(.., search_type)| search_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("text search configuration foo", TextSearch::Configuration(vec!["foo".into()]))]
    #[test_case("text search dictionary foo", TextSearch::Dictionary(vec!["foo".into()]))]
    #[test_case("text search parser foo", TextSearch::Parser(vec!["foo".into()]))]
    #[test_case("text search template foo", TextSearch::Template(vec!["foo".into()]))]
    fn test_text_search(source: &str, expected: TextSearch) {
        test_parser!(source, text_search, expected)
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Configuration;
use pg_lexer::Keyword::Dictionary;
use pg_lexer::Keyword::ParserKw;
use pg_lexer::Keyword::Search;
use pg_lexer::Keyword::Template;
use pg_lexer::Keyword::Text;
