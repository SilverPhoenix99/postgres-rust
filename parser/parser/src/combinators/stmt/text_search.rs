#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum TextSearch {
    Configuration(QualifiedName),
    Dictionary(QualifiedName),
    Parser(QualifiedName),
    Template(QualifiedName),
}

pub(super) fn text_search(stream: &mut TokenStream) -> scan::Result<TextSearch> {

    /*
        TEXT SEARCH (
              CONFIGURATION
            | DICTIONARY
            | PARSER
            | TEMPLATE
         ) any_name
    */

    let (.., search_type) = seq!(
        Text,
        Search,
        alt!(
            seq!(Configuration, any_name)
                .map(|(_, name)| TextSearch::Configuration(name)),
            seq!(Dictionary, any_name)
                .map(|(_, name)| TextSearch::Dictionary(name)),
            seq!(ParserKw, any_name)
                .map(|(_, name)| TextSearch::Parser(name)),
            seq!(Template, any_name)
                .map(|(_, name)| TextSearch::Template(name))
        )
    ).parse(stream)?;

    Ok(search_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
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
use crate::combinators::foundation::alt;
use pg_basics::QualifiedName;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Configuration;
use pg_lexer::Keyword::Dictionary;
use pg_lexer::Keyword::ParserKw;
use pg_lexer::Keyword::Search;
use pg_lexer::Keyword::Template;
use pg_lexer::Keyword::Text;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
