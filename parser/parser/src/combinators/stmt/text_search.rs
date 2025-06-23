#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum TextSearch {
    Configuration(QualifiedName),
    Dictionary(QualifiedName),
    Parser(QualifiedName),
    Template(QualifiedName),
}

pub(super) fn text_search() -> impl Combinator<Output = TextSearch> {

    /*
        TEXT SEARCH (
              CONFIGURATION
            | DICTIONARY
            | PARSER
            | TEMPLATE
         ) any_name
    */

    (Text, Search)
        .and_right(match_first! {
            Configuration
                .and_right(any_name)
                .map(TextSearch::Configuration),
            Dictionary
                .and_right(any_name)
                .map(TextSearch::Dictionary),
            ParserKw
                .and_right(any_name)
                .map(TextSearch::Parser),
            Template
                .and_right(any_name)
                .map(TextSearch::Template)
        })
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
        test_parser!(source, text_search(), expected)
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Configuration;
use pg_lexer::Keyword::Dictionary;
use pg_lexer::Keyword::ParserKw;
use pg_lexer::Keyword::Search;
use pg_lexer::Keyword::Template;
use pg_lexer::Keyword::Text;
