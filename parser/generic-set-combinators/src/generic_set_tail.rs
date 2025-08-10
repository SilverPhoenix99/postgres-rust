/// Alias: `generic_set`
pub fn generic_set_tail(ctx: &mut ParserContext) -> scan::Result<ValueOrDefault<Vec<VarValue>>> {

    /*
          (TO | '=') DEFAULT
        | (TO | '=') var_list
    */

    let (_, value) = seq!(
        alt!(
            To.skip(),
            Equals.skip()
        ),
        alt!(
            DefaultKw.map(|_| ValueOrDefault::Default),
            var_list.map(ValueOrDefault::Value)
        )
    ).parse(ctx)?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("TO Default", ValueOrDefault::Default)]
    #[test_case("= Default", ValueOrDefault::Default)]
    #[test_case("TO true, 'off'", ValueOrDefault::Value(vec![VarValue::Boolean(true), VarValue::String("off".into())]))]
    #[test_case("= false, 'on'", ValueOrDefault::Value(vec![VarValue::Boolean(false), VarValue::String("on".into())]))]
    fn test_generic_set_tail(source: &str, expected: ValueOrDefault<Vec<VarValue>>) {
        test_parser!(source, generic_set_tail, expected)
    }
}

use crate::var_list;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_generic_set_ast::VarValue;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::To;
use pg_lexer::OperatorKind::Equals;
use pg_parser_core::scan;
use pg_sink_ast::ValueOrDefault;
