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
    use test_case::test_matrix;

    #[test_matrix("true" => Ok(true.into()))]
    #[test_matrix("false" => Ok(false.into()))]
    #[test_matrix("on" => Ok("on".into()))]
    #[test_matrix("off" => Ok("off".into()))]
    #[test_matrix("'value'" => Ok("value".into()))]
    #[test_matrix("+123" => Ok(123.into()))]
    fn test_var_value(source: &str) -> scan::Result<VarValue> {
        test_parser!(source, var_value)
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
