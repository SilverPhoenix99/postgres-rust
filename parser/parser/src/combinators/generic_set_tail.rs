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
    use test_case::test_matrix;

    #[test_matrix("TO Default" => Ok(DefaultableValue::Default))]
    #[test_matrix("= Default" => Ok(DefaultableValue::Default))]
    #[test_matrix("TO null" => Ok(DefaultableValue::Null))]
    #[test_matrix("= null" => Ok(DefaultableValue::Null))]
    #[test_matrix("TO true, 'off'" => Ok(DefaultableValue::Value(vec![VarValue::Boolean(true), VarValue::String("off".into())])))]
    #[test_matrix("= false, 'on'" => Ok(DefaultableValue::Value(vec![VarValue::Boolean(false), VarValue::String("on".into())])))]
    fn test_generic_set_tail(source: &str) -> scan::Result<DefaultableValue<Vec<VarValue>>> {
        test_parser!(source, generic_set_tail)
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
