/// Alias: `CreateConversionStmt`
pub(super) fn create_conversion_stmt(stream: &mut TokenStream) -> scan::Result<CreateConversionStmt> {

    /*
        ( DEFAULT )? CONVERSION_P any_name FOR SCONST TO SCONST FROM any_name
    */

    let (is_default, name, _, for_encoding, _, to_encoding, _, function) = seq!(
        alt!(
            seq!(DefaultKw, Conversion).map(|_| true),
            Conversion.map(|_| false)
        ),
        any_name,
        For,
        string,
        To,string,
        FromKw,
        any_name
    ).parse(stream)?;

    let stmt = CreateConversionStmt::new(
        name,
        for_encoding,
        to_encoding,
        function,
        is_default
    );

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("conversion foo for 'bar' to 'baz' from qux",
        CreateConversionStmt::new(vec!["foo".into()], "bar", "baz", vec!["qux".into()], false)
    )]
    #[test_case("default conversion foo for 'bar' to 'baz' from qux",
        CreateConversionStmt::new(vec!["foo".into()], "bar", "baz", vec!["qux".into()], true)
    )]
    fn test_create_conversion_stmt(source: &str, expected: CreateConversionStmt) {
        test_parser!(source, create_conversion_stmt, expected);
    }
}

use crate::combinators::any_name;
use pg_ast::CreateConversionStmt;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Conversion;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::To;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
