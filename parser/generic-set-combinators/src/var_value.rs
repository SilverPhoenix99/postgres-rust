pub fn var_list(ctx: &mut ParserContext) -> scan::Result<Vec<VarValue>> {

    many!(sep = Comma, var_value).parse(ctx)
}

/// Alias: `utility_option_arg`
pub fn var_value(ctx: &mut ParserContext) -> scan::Result<VarValue> {

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
    use pg_combinators::test_parser;
    use pg_generic_set_ast::VarValue;
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

use pg_combinators::alt;
use pg_combinators::many;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_generic_set_ast::VarValue;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_sink_combinators::boolean_or_string;
use pg_sink_combinators::signed_number;
