/// Alias: `copy_generic_opt_arg_list`
pub(super) fn boolean_or_string_list(stream: &mut TokenStream) -> scan::Result<Vec<BooleanOrString>> {

    many_sep(Comma, boolean_or_string).parse(stream)
}

/// Alias: `opt_boolean_or_string`
pub(super) fn boolean_or_string(stream: &mut TokenStream) -> scan::Result<BooleanOrString> {

    alt!(
        True.map(|_| true.into()),
        False.map(|_| false.into()),
        On.map(|kw| kw.text().into()),
        string.map(From::from),
        // `Off` is handled by this production:
        non_reserved_word.map(From::from),
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("true", true.into())]
    #[test_case("false", false.into())]
    #[test_case("on", "on".into())]
    #[test_case("off", "off".into())]
    #[test_case("'value'", "value".into())]
    fn test_boolean_or_string(source: &str, expected: BooleanOrString) {
        test_parser!(source, boolean_or_string, expected)
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::non_reserved_word;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::BooleanOrString;
use pg_lexer::Keyword::False;
use pg_lexer::Keyword::On;
use pg_lexer::Keyword::True;
use pg_lexer::OperatorKind::Comma;
