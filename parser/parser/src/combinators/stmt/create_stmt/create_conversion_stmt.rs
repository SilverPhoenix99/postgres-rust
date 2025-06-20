/// Alias: `CreateConversionStmt`
pub(super) fn create_conversion_stmt() -> impl Combinator<Output = CreateConversionStmt> {

    /*
        opt_default CONVERSION_P any_name FOR SCONST TO SCONST FROM any_name
    */

    sequence!(
        or(
            and(DefaultKw, Conversion).map(|_| true),
            Conversion.map(|_| false)
        ),
        any_name(),
        For.and_right(parser(string)),
        To.and_right(parser(string)),
        FromKw.and_right(any_name())
    )
        .map(|(is_default, name, for_encoding, to_encoding, function)| {
            CreateConversionStmt::new(
                name,
                for_encoding,
                to_encoding,
                function,
                is_default
            )
        })
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
        test_parser!(source, create_conversion_stmt(), expected);
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::and;
use crate::combinators::foundation::or;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::CreateConversionStmt;
use pg_lexer::Keyword::Conversion;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::To;
