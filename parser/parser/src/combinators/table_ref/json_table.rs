fn json_table_column_path_clause(stream: &mut TokenStream) -> scan::Result<Box<str>> {

    /*
        PATH SCONST
    */

    let (_, string) = seq!(Path, string)
        .parse(stream)?;

    Ok(string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("path 'foo'" => Ok("foo".into()))]
    fn test_json_table_column_path_clause(source: &str) -> scan::Result<Box<str>> {
        test_parser!(source, json_table_column_path_clause)
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Path;
