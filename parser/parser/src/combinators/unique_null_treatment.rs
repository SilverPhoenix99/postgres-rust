/// Alias: `opt_unique_null_treatment`
pub(super) fn unique_null_treatment(ctx: &mut ParserContext) -> scan::Result<UniqueNullTreatment> {

    /*
        NULLS ( NOT )? DISTINCT
    */

    let (_, not, _) = seq!(
        Nulls,
        Not.optional(),
        Distinct
    ).parse(ctx)?;

    let nulls = UniqueNullTreatment::from(not.is_none());

    Ok(nulls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("nulls distinct" => Ok(UniqueNullTreatment::NullsDistinct))]
    #[test_matrix("nulls not distinct" => Ok(UniqueNullTreatment::NullsNotDistinct))]
    fn test_unique_null_treatment(source: &str) -> scan::Result<UniqueNullTreatment> {
        test_parser!(source, unique_null_treatment)
    }
}

use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::UniqueNullTreatment;
use pg_lexer::Keyword::Distinct;
use pg_lexer::Keyword::Not;
use pg_lexer::Keyword::Nulls;
use pg_parser_core::scan;
