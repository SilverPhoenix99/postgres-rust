/// Alias: `NonReservedWord_or_Sconst`
pub(super) fn non_reserved_word_or_sconst() -> impl Combinator<Output = Str> {

    non_reserved_word().or(string().map(Boxed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("action", Str::Static("action"))]
    #[test_case("'some_string'", Str::Static("some_string"))]
    fn test_non_reserved_word_or_sconst(source: &str, expected: Str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = non_reserved_word_or_sconst().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::non_reserved_word;
use postgres_basics::Str;
use postgres_basics::Str::Boxed;
