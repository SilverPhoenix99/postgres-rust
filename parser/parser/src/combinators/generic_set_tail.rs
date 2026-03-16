/// Alias: `generic_set`
pub(super) fn generic_set_tail(ctx: &mut ParserContext) -> scan::Result<DefaultableValue<Vec<VarValue>>> {

    /*
          (TO | '=') DEFAULT
        | (TO | '=') NULL
        | (TO | '=') var_list
    */

    let (_, value) = seq!(
        alt!(
            To.skip(),
            Equals.skip()
        ),
        alt!(
            DefaultKw.map(|_| DefaultableValue::Default),
            Null.map(|_| DefaultableValue::Null),
            var_list.map(DefaultableValue::Value)
        )
    ).parse(ctx)?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("TO Default", DefaultableValue::Default)]
    #[test_case("= Default", DefaultableValue::Default)]
    #[test_case("TO null", DefaultableValue::Null)]
    #[test_case("= null", DefaultableValue::Null)]
    #[test_case("TO true, 'off'", DefaultableValue::Value(vec![VarValue::Boolean(true), VarValue::String("off".into())]))]
    #[test_case("= false, 'on'", DefaultableValue::Value(vec![VarValue::Boolean(false), VarValue::String("on".into())]))]
    fn test_generic_set_tail(source: &str, expected: DefaultableValue<Vec<VarValue>>) {
        test_parser!(source, generic_set_tail, expected)
    }
}

use super::var_list;
use crate::alt;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::DefaultableValue;
use pg_ast::VarValue;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::Null;
use pg_lexer::Keyword::To;
use pg_lexer::OperatorKind::Equals;
use pg_parser_core::scan;
