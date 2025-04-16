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
        For.and_right(string()),
        To.and_right(string()),
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
    use crate::parser::tests::test_parser;
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

use crate::lexer::Keyword::Conversion;
use crate::lexer::Keyword::DefaultKw;
use crate::lexer::Keyword::For;
use crate::lexer::Keyword::FromKw;
use crate::lexer::Keyword::To;
use crate::parser::ast_node::CreateConversionStmt;
use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::string;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
