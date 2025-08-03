pub(super) fn generic_set_tail(stream: &mut TokenStream) -> scan::Result<ValueOrDefault<Vec<VarValue>>> {

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
    ).parse(stream)?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("TO Default", ValueOrDefault::Default)]
    #[test_case("= Default", ValueOrDefault::Default)]
    #[test_case("TO true, 'off'", ValueOrDefault::Value(vec![VarValue::Boolean(true), VarValue::String("off".into())]))]
    #[test_case("= false, 'on'", ValueOrDefault::Value(vec![VarValue::Boolean(false), VarValue::String("on".into())]))]
    fn test_generic_set_tail(source: &str, expected: ValueOrDefault<Vec<VarValue>>) {
        test_parser!(source, generic_set_tail, expected)
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::var_list;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ValueOrDefault;
use pg_ast::VarValue;
use pg_lexer::Keyword::DefaultKw;
use pg_lexer::Keyword::To;
use pg_lexer::OperatorKind::Equals;
