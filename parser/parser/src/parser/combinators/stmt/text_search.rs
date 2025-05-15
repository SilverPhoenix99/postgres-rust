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

    and(Text, Search)
        .and_right(match_first! {
            Configuration
                .and_right(any_name())
                .map(TextSearch::Configuration),
            Dictionary
                .and_right(any_name())
                .map(TextSearch::Dictionary),
            ParserKw
                .and_right(any_name())
                .map(TextSearch::Parser),
            Template
                .and_right(any_name())
                .map(TextSearch::Template)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test_case("text search configuration foo", TextSearch::Configuration(vec!["foo".into()]))]
    #[test_case("text search dictionary foo", TextSearch::Dictionary(vec!["foo".into()]))]
    #[test_case("text search parser foo", TextSearch::Parser(vec!["foo".into()]))]
    #[test_case("text search template foo", TextSearch::Template(vec!["foo".into()]))]
    fn test_text_search(source: &str, expected: TextSearch) {
        test_parser!(source, text_search(), expected)
    }
}

use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::QualifiedName;
use postgres_parser_lexer::Keyword::Configuration;
use postgres_parser_lexer::Keyword::Dictionary;
use postgres_parser_lexer::Keyword::ParserKw;
use postgres_parser_lexer::Keyword::Search;
use postgres_parser_lexer::Keyword::Template;
use postgres_parser_lexer::Keyword::Text;
