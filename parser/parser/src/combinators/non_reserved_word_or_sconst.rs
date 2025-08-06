/// Alias: `NonReservedWord_or_Sconst`
pub(super) fn non_reserved_word_or_sconst(stream: &mut TokenStream) -> scan::Result<Str> {

    alt!(
        non_reserved_word,
        string.map(Str::from)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("action", Str::Static("action"))]
    #[test_case("'some_string'", Str::Static("some_string"))]
    fn test_non_reserved_word_or_sconst(source: &str, expected: Str) {
        test_parser!(source, non_reserved_word_or_sconst, expected)
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::non_reserved_word;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_parser_core::scan;
