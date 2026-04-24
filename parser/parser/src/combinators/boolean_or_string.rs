/// Alias: `copy_generic_opt_arg_list`
pub(super) fn boolean_or_string_list(ctx: &mut ParserContext) -> scan::Result<Vec<BooleanOrString>> {

    many!(sep = Comma, boolean_or_string).parse(ctx)
}

/// Alias: `opt_boolean_or_string`
pub(super) fn boolean_or_string(ctx: &mut ParserContext) -> scan::Result<BooleanOrString> {

    alt!(
        True.map(|_| true.into()),
        False.map(|_| false.into()),
        On.map(|kw| kw.text().into()),
        string.map(From::from),
        // `Off` is handled by this production:
        non_reserved_word.map(From::from),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("true" => Ok(true.into()))]
    #[test_matrix("false" => Ok(false.into()))]
    #[test_matrix("on" => Ok("on".into()))]
    #[test_matrix("off" => Ok("off".into()))]
    #[test_matrix("'value'" => Ok("value".into()))]
    fn test_boolean_or_string(source: &str) -> scan::Result<BooleanOrString> {
        test_parser!(source, boolean_or_string)
    }
}

use crate::alt;
use crate::combinators::core::string;
use crate::combinators::core::Combinator;
use crate::combinators::non_reserved_word;
use crate::many;
use crate::ParserContext;
use pg_ast::BooleanOrString;
use pg_lexer::Keyword::False;
use pg_lexer::Keyword::On;
use pg_lexer::Keyword::True;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
