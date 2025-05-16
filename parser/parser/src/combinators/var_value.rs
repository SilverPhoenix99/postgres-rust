pub(super) fn var_list() -> impl Combinator<Output = Vec<VarValue>> {

    many_sep(Comma, var_value())
}

pub(super) fn var_value() -> impl Combinator<Output = VarValue> {

    /*
          opt_boolean_or_string
        | signed_number
    */

    match_first!(
        boolean_or_string().map(From::from),
        signed_number().map(From::from)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("true", true.into())]
    #[test_case("false", false.into())]
    #[test_case("on", "on".into())]
    #[test_case("off", "off".into())]
    #[test_case("'value'", "value".into())]
    #[test_case("+123", 123.into())]
    fn test_var_value(source: &str, expected: VarValue) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = var_value().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::boolean_or_string;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::signed_number;
use postgres_parser_ast::VarValue;
use postgres_parser_lexer::OperatorKind::Comma;
