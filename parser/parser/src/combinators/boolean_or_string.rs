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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
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

use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::non_reserved_word;
use pg_ast::BooleanOrString;
use pg_lexer::Keyword::False;
use pg_lexer::Keyword::On;
use pg_lexer::Keyword::True;
use pg_lexer::OperatorKind::Comma;
