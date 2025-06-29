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

    let (.., search_type) = (
        Text,
        Search,
        or((
            (Configuration, any_name)
                .map(|(_, name)| TextSearch::Configuration(name)),
            (Dictionary, any_name)
                .map(|(_, name)| TextSearch::Dictionary(name)),
            (ParserKw, any_name)
                .map(|(_, name)| TextSearch::Parser(name)),
            (Template, any_name)
                .map(|(_, name)| TextSearch::Template(name))
        ))
    ).parse(stream)?;

    Ok(search_type)
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
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Configuration;
use pg_lexer::Keyword::Dictionary;
use pg_lexer::Keyword::ParserKw;
use pg_lexer::Keyword::Search;
use pg_lexer::Keyword::Template;
use pg_lexer::Keyword::Text;
