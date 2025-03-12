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
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("", UniqueNullTreatment::NullsDistinct)]
    #[test_case("nulls distinct", UniqueNullTreatment::NullsDistinct)]
    #[test_case("nulls not distinct", UniqueNullTreatment::NullsNotDistinct)]
    fn test_opt_unique_null_treatment(source: &str, expected: UniqueNullTreatment) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), opt_unique_null_treatment().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Distinct;
use crate::lexer::Keyword::Not;
use crate::lexer::Keyword::Nulls;
use crate::parser::ast_node::UniqueNullTreatment;
use crate::parser::combinators::foundation::optional;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
