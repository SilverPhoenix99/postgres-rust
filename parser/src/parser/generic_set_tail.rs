pub(super) fn generic_set_tail() -> impl Combinator<Output = VarList> {

    To.skip().or(Equals.skip())
        .and_right(match_first!(
            DefaultKw.map(|_| VarList::Default),
            var_list().map(VarList::Values)
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::VarValue;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("TO Default", VarList::Default)]
    #[test_case("= Default", VarList::Default)]
    #[test_case("TO true, 'off'", VarList::Values(vec![VarValue::Boolean(true), VarValue::String("off".into())]))]
    #[test_case("= false, 'on'", VarList::Values(vec![VarValue::Boolean(false), VarValue::String("on".into())]))]
    fn test_generic_set_tail(source: &str, expected: VarList) {

        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = generic_set_tail().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::lexer::Keyword::DefaultKw;
use crate::lexer::Keyword::To;
use crate::lexer::OperatorKind::Equals;
use crate::parser::ast_node::VarList;
use crate::parser::combinators::match_first;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::var_value::var_list;
