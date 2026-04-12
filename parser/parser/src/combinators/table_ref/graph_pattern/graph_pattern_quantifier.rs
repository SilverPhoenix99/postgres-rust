/// Alias: `opt_graph_pattern_quantifier`
pub(super) fn graph_pattern_quantifier(ctx: &mut ParserContext) -> scan::Result<RangeInclusive<NonNegative>> {

    /*
        | '{' ',' ICONST '}'        => (0,   max)
        | '{' ICONST ',' ICONST '}' => (min, max)
        | '{' ICONST '}'            => (min, min)
    */

    braces!(
        alt!(
            seq!(Comma, integer)
                .map(|(_, max)|
                    RangeInclusive::new(NonNegative::default(), max)
                ),
            seq!(
                integer,
                seq!(Comma, integer).map(|(_, max)| max).optional()
            )
                .map(|(min, max)| {
                    let max = max.unwrap_or(min);
                    RangeInclusive::new(min, max)
                })
        )
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("{,1}" => Ok(range(0, 1)))]
    #[test_case("{2,3}" => Ok(range(2, 3)))]
    #[test_case("{4}" => Ok(range(4, 4)))]
    fn test_graph_pattern_quantifier(source: &str) -> scan::Result<RangeInclusive<NonNegative>> {
        test_parser!(source, graph_pattern_quantifier)
    }

    fn range<S: Into<NonNegative>, E: Into<NonNegative>>(start: S, end: E) -> RangeInclusive<NonNegative> {
        RangeInclusive::new(start.into(), end.into())
    }
}

use crate::alt;
use crate::braces;
use crate::combinators::core::integer;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use crate::seq;
use core::ops::RangeInclusive;
use pg_basics::NonNegative;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
