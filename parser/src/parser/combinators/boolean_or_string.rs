/// Alias: `copy_generic_opt_arg_list`
pub(super) fn boolean_or_string_list() -> impl Combinator<Output = Vec<BooleanOrString>> {

    many_sep(Comma, boolean_or_string())
}

/// Alias: `opt_boolean_or_string`
pub(super) fn boolean_or_string() -> impl Combinator<Output = BooleanOrString> {

    match_first!(
        True.map(|_| true.into()),
        False.map(|_| false.into()),
        On.map(|kw| kw.text().into()),
        string().map(From::from),
        // `Off` is handled by this production:
        non_reserved_word().map(From::from),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("true", true.into())]
    #[test_case("false", false.into())]
    #[test_case("on", "on".into())]
    #[test_case("off", "off".into())]
    #[test_case("'value'", "value".into())]
    fn test_opt_boolean_or_string(source: &str, expected: BooleanOrString) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = boolean_or_string().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::lexer::Keyword::False;
use crate::lexer::Keyword::On;
use crate::lexer::Keyword::True;
use crate::lexer::OperatorKind::Comma;
use crate::parser::ast_node::BooleanOrString;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::string;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::non_reserved_word;
