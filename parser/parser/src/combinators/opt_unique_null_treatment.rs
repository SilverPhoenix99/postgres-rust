pub(super) fn opt_unique_null_treatment() -> impl Combinator<Output = UniqueNullTreatment> {

    Nulls
        .and_then(optional(Not), |_, not| not.is_none().into())
        .and_left(Distinct)
        .optional()
        .map(Option::unwrap_or_default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("", UniqueNullTreatment::NullsDistinct)]
    #[test_case("nulls distinct", UniqueNullTreatment::NullsDistinct)]
    #[test_case("nulls not distinct", UniqueNullTreatment::NullsNotDistinct)]
    fn test_opt_unique_null_treatment(source: &str, expected: UniqueNullTreatment) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), opt_unique_null_treatment().parse(&mut stream));
    }
}

use crate::combinators::foundation::optional;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::UniqueNullTreatment;
use pg_lexer::Keyword::Distinct;
use pg_lexer::Keyword::Not;
use pg_lexer::Keyword::Nulls;
