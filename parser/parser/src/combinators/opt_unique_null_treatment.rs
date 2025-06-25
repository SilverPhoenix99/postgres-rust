pub(super) fn opt_unique_null_treatment(stream: &mut TokenStream) -> Result<UniqueNullTreatment> {

    /*
        ( NULLS (NOT)? DISTINCT )?
    */

    let nulls = seq!(stream =>
        Nulls,
        Not.optional()
            .map(|not| not.is_none()),
        Distinct
    );

    let nulls = match nulls.optional()? {
        Some((_, nulls, _)) => nulls.into(),
        None => Default::default()
    };

    Ok(nulls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("", UniqueNullTreatment::NullsDistinct)]
    #[test_case("nulls distinct", UniqueNullTreatment::NullsDistinct)]
    #[test_case("nulls not distinct", UniqueNullTreatment::NullsNotDistinct)]
    fn test_opt_unique_null_treatment(source: &str, expected: UniqueNullTreatment) {
        test_parser!(source, opt_unique_null_treatment, expected)
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::UniqueNullTreatment;
use pg_lexer::Keyword::Distinct;
use pg_lexer::Keyword::Not;
use pg_lexer::Keyword::Nulls;
