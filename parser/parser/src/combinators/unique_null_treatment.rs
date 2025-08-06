/// Alias: `opt_unique_null_treatment`
pub(super) fn unique_null_treatment(stream: &mut TokenStream) -> scan::Result<UniqueNullTreatment> {

    /*
        NULLS ( NOT )? DISTINCT
    */

    let (_, not, _) = seq!(
        Nulls,
        Not.optional(),
        Distinct
    ).parse(stream)?;

    let nulls = UniqueNullTreatment::from(not.is_none());

    Ok(nulls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("nulls distinct", UniqueNullTreatment::NullsDistinct)]
    #[test_case("nulls not distinct", UniqueNullTreatment::NullsNotDistinct)]
    fn test_unique_null_treatment(source: &str, expected: UniqueNullTreatment) {
        test_parser!(source, unique_null_treatment, expected)
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::stream::TokenStream;
use pg_ast::UniqueNullTreatment;
use pg_lexer::Keyword::Distinct;
use pg_lexer::Keyword::Not;
use pg_lexer::Keyword::Nulls;
use pg_parser_core::scan;
