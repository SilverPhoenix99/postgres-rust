pub(super) fn path_factor(ctx: &mut ParserContext) -> scan::Result<GraphElementPatternKind> {

    /*
        path_primary ( graph_pattern_quantifier )?
    */

    let (mut primary, quantifier) = seq!(
        path_primary,
        graph_pattern_quantifier.optional()
    ).parse(ctx)?;

    primary.set_quantifier(quantifier);

    Ok(primary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        core::ops::RangeInclusive,
        pg_ast::GraphElementPattern,
        pg_basics::NonNegative
    };

    #[test_case("->" => Ok(
        GraphElementPatternKind::EdgePatternRight(
            GraphElementPattern::default()
        )
    ))]
    #[test_case("<-{1}" => Ok(
        GraphElementPatternKind::EdgePatternLeft(
            GraphElementPattern::default()
                .with_quantifier(RangeInclusive::new(NonNegative::from(1), NonNegative::from(1)))
        )
    ))]
    fn test_path_factor(source: &str) -> scan::Result<GraphElementPatternKind> {
        test_parser!(source, path_factor)
    }
}

use super::graph_pattern_quantifier;
use super::path_primary;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::GraphElementPatternKind;
use pg_parser_core::scan;
