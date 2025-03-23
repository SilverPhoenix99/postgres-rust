pub(super) fn generic_set_tail() -> impl Combinator<Output = ValueOrDefault<Vec<VarValue>>> {

    /*
          (TO | '=') DEFAULT
        | (TO | '=') var_list
    */

    To.skip().or(Equals.skip())
        .and_right(match_first!(
            DefaultKw.map(|_| ValueOrDefault::Default),
            var_list().map(ValueOrDefault::Value)
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("TO Default", ValueOrDefault::Default)]
    #[test_case("= Default", ValueOrDefault::Default)]
    #[test_case("TO true, 'off'", ValueOrDefault::Value(vec![VarValue::Boolean(true), VarValue::String("off".into())]))]
    #[test_case("= false, 'on'", ValueOrDefault::Value(vec![VarValue::Boolean(false), VarValue::String("on".into())]))]
    fn test_generic_set_tail(source: &str, expected: ValueOrDefault<Vec<VarValue>>) {

        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = generic_set_tail().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::lexer::Keyword::DefaultKw;
use crate::lexer::Keyword::To;
use crate::lexer::OperatorKind::Equals;
use crate::parser::ast_node::ValueOrDefault;
use crate::parser::ast_node::VarValue;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::var_list;
