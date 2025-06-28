/// Alias: `CreateConversionStmt`
pub(super) fn create_conversion_stmt(stream: &mut TokenStream) -> scan::Result<CreateConversionStmt> {

    /*
        opt_default CONVERSION_P any_name FOR SCONST TO SCONST FROM any_name
    */

    let (is_default, name, _, for_encoding, _, to_encoding, _, function) = seq!(=>
        choice!(stream =>
            seq!(stream => DefaultKw, Conversion).map(|_| true),
            Conversion.parse(stream).map(|_| false)
        ),
        any_name(stream),
        For.parse(stream),
        string(stream),
        To.parse(stream),
        string(stream),
        FromKw.parse(stream),
        any_name(stream)
    )?;

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
    use crate::tests::test_parser;
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
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::CreateConversionStmt;
use pg_lexer::Keyword::Conversion;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::To;
