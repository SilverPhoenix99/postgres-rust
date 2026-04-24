/// Alias: `NonReservedWord_or_Sconst`
pub(super) fn non_reserved_word_or_sconst(ctx: &mut ParserContext) -> scan::Result<Str> {

    alt!(
        non_reserved_word,
        string.map(Str::from)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("action" => Ok("action".into()))]
    #[test_matrix("'some_string'" => Ok("some_string".into()))]
    fn test_non_reserved_word_or_sconst(source: &str) -> scan::Result<Str> {
        test_parser!(source, non_reserved_word_or_sconst)
    }
}

use crate::alt;
use crate::combinators::core::string;
use crate::combinators::core::Combinator;
use crate::combinators::non_reserved_word;
use crate::ParserContext;
use pg_basics::Str;
use pg_parser_core::scan;
