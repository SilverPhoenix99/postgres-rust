/// Alias: `json_returning_clause_opt`
pub(super) fn json_returning_clause(stream: &mut TokenStream) -> scan::Result<JsonOutput> {

    /*
        RETURNING Typename ( json_format_clause )?
    */

    let (_, type_name, format) = (
        Returning,
        typename,
        json_format_clause.optional()
    ).parse(stream)?;

    let format = format.unwrap_or_default();
    let output = JsonOutput::new(type_name, format);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        JsonEncoding::UTF8,
        JsonFormat,
        TypeName::Json,
    };
    use test_case::test_case;

    #[test_case("returning json" => Ok(
        JsonOutput::new(Json, JsonFormat::default())
    ))]
    #[test_case("returning json format json" => Ok(
        JsonOutput::new(Json, JsonFormat::text())
    ))]
    #[test_case("returning json format json encoding utf8" => Ok(
        JsonOutput::new(
            Json,
            JsonFormat::text()
                .with_encoding(UTF8)
        )
    ))]
    fn test_json_returning_clause(source: &str) -> scan::Result<JsonOutput> {
        test_parser!(source, json_returning_clause)
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::json_format_clause;
use crate::combinators::typename;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::JsonOutput;
use pg_lexer::Keyword::Returning;
