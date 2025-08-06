pub(super) fn var_list(stream: &mut TokenStream) -> scan::Result<Vec<VarValue>> {

    many!(sep = Comma, var_value).parse(stream)
}

pub(super) fn var_value(stream: &mut TokenStream) -> scan::Result<VarValue> {

    /*
          boolean_or_string
        | signed_number
    */

    alt!(
        boolean_or_string.map(From::from),
        signed_number.map(From::from)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("true", true.into())]
    #[test_case("false", false.into())]
    #[test_case("on", "on".into())]
    #[test_case("off", "off".into())]
    #[test_case("'value'", "value".into())]
    #[test_case("+123", 123.into())]
    fn test_var_value(source: &str, expected: VarValue) {
        test_parser!(source, var_value, expected)
    }
}

use crate::combinators::boolean_or_string;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::many;
use crate::combinators::foundation::Combinator;
use crate::combinators::signed_number;
use crate::stream::TokenStream;
use pg_ast::VarValue;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
