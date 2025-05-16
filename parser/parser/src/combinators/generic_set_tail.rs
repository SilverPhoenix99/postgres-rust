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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
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

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::var_list;
use postgres_parser_ast::ValueOrDefault;
use postgres_parser_ast::VarValue;
use postgres_parser_lexer::Keyword::DefaultKw;
use postgres_parser_lexer::Keyword::To;
use postgres_parser_lexer::OperatorKind::Equals;
