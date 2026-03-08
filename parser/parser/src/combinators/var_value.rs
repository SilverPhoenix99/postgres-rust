pub(super) fn var_list(ctx: &mut ParserContext) -> scan::Result<Vec<VarValue>> {

    many!(sep = Comma, var_value).parse(ctx)
}

/// Alias: `utility_option_arg`
pub(super) fn var_value(ctx: &mut ParserContext) -> scan::Result<VarValue> {

    /*
          boolean_or_string
        | signed_number
    */

    alt!(
        boolean_or_string.map(From::from),
        signed_number.map(From::from)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
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

use crate::alt;
use crate::combinators::boolean_or_string;
use crate::combinators::core::Combinator;
use crate::combinators::signed_number;
use crate::many;
use crate::ParserContext;
use pg_ast::VarValue;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
