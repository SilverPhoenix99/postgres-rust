fn json_table_column_definition_list(stream: &mut TokenStream) -> scan::Result<Vec<()>> {

    /*
        json_table_column_definition ( ',' json_table_column_definition )*
    */

    many_sep(Comma, json_table_column_definition)
        .parse(stream)
}

fn json_table_column_definition(stream: &mut TokenStream) -> scan::Result<()> {

    /*
          NESTED ( PATH )? SCONST ( AS ColId )? COLUMNS '(' json_table_column_definition_list ')'
        | ColId FOR ORDINALITY
        | ColId Typename EXISTS ( json_table_column_path_clause )? ( json_on_error_clause )?
        | ColId Typename
            ( json_format_clause )?
            ( json_table_column_path_clause )?
            json_wrapper_behavior
            ( json_quotes_clause )?
            ( json_behavior_clause )?
    */

    todo!()
}

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

use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::JsonTablePathSpec;
use pg_lexer::Keyword::Path;
use pg_lexer::OperatorKind::Comma;
