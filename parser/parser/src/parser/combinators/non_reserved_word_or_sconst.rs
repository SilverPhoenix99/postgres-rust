/// Alias: `NonReservedWord_or_Sconst`
pub(super) fn non_reserved_word_or_sconst() -> impl Combinator<Output = Str> {

    non_reserved_word().or(string().map(Boxed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("action", Str::Static("action"))]
    #[test_case("'some_string'", Str::Static("some_string"))]
    fn test_non_reserved_word_or_sconst(source: &str, expected: Str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = non_reserved_word_or_sconst().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::parser::combinators::foundation::string;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::non_reserved_word;
use postgres_basics::Str;
use postgres_basics::Str::Boxed;
