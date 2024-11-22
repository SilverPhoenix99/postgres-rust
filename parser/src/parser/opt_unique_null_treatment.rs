fn opt_unique_null_treatment() -> impl Combinator<Output = UniqueNullTreatment> {
    use crate::lexer::Keyword::{Distinct, Not, Nulls};
    use crate::parser::combinators::{keyword, optional, CombinatorHelpers};

    keyword(Nulls)
        .and_then(optional(keyword(Not)), |_, not| not.is_none().into())
        .and_left(keyword(Distinct))
        .optional()
        .map(Option::unwrap_or_default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
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

use crate::parser::ast_node::UniqueNullTreatment;
use crate::parser::combinators::Combinator;
