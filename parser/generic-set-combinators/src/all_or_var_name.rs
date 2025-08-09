/// Alias: `generic_reset`
pub fn all_or_var_name(stream: &mut TokenStream) -> scan::Result<OneOrAll<QualifiedName>> {

    /*
          ALL
        | var_name
    */

    alt!(
        Keyword::All.map(|_| OneOrAll::All),
        var_name.map(OneOrAll::One)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use pg_sink_ast::OneOrAll;
    use test_case::test_case;

    #[test_case("all" => Ok(OneOrAll::All))]
    #[test_case("_ident" => Ok(OneOrAll::One(vec!["_ident".into()])))]
    fn test_all_or_var_name(source: &str) -> scan::Result<OneOrAll<QualifiedName>> {
        test_parser!(source, all_or_var_name)
    }
}

use pg_basics::QualifiedName;
use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_lexer::Keyword;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_ast::OneOrAll;
use pg_sink_combinators::var_name;
