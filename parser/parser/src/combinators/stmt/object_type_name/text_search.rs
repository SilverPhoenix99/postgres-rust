#[derive(Debug, Clone, Eq, PartialEq)]
pub(in crate::combinators::stmt) enum TextSearch {
    Configuration(QualifiedName),
    Dictionary(QualifiedName),
    Parser(QualifiedName),
    Template(QualifiedName),
}

pub(in crate::combinators::stmt) fn text_search(ctx: &mut ParserContext) -> scan::Result<TextSearch> {

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
    ).parse(ctx)?;

    Ok(search_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("text search configuration foo" => Ok(TextSearch::Configuration(vec!["foo".into()])))]
    #[test_matrix("text search dictionary foo" => Ok(TextSearch::Dictionary(vec!["foo".into()])))]
    #[test_matrix("text search parser foo" => Ok(TextSearch::Parser(vec!["foo".into()])))]
    #[test_matrix("text search template foo" => Ok(TextSearch::Template(vec!["foo".into()])))]
    fn test_text_search(source: &str) -> scan::Result<TextSearch> {
        test_parser!(source, text_search)
    }
}

use crate::alt;
use crate::combinators::any_name;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Configuration;
use pg_lexer::Keyword::Dictionary;
use pg_lexer::Keyword::ParserKw;
use pg_lexer::Keyword::Search;
use pg_lexer::Keyword::Template;
use pg_lexer::Keyword::Text;
use pg_parser_core::scan;
