fn json_table_column_path_clause(stream: &mut TokenStream) -> scan::Result<JsonTablePathSpec> {

    /*
        PATH SCONST
    */

    let (_, path_spec) = seq!(Path, string)
        .parse(stream)?;

    let path_spec = JsonTablePathSpec::new(path_spec);
    Ok(path_spec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("path 'foo'" => Ok(JsonTablePathSpec::new("foo")))]
    fn test_json_table_column_path_clause(source: &str) -> scan::Result<JsonTablePathSpec> {
        test_parser!(source, json_table_column_path_clause)
    }
}

use pg_ast::JsonTablePathSpec;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Path;
