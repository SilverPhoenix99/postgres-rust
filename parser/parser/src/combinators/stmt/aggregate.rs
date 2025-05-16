pub(super) fn aggregate() -> impl Combinator<Output = AggregateWithArgs> {

    /*
        AGGREGATE aggregate_with_argtypes
    */

    Aggregate
        .and_right(aggregate_with_argtypes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_aggregate() {
        test_parser!(
            source = "aggregate foo(*)",
            parser = aggregate(),
            expected = AggregateWithArgs::new(vec!["foo".into()], vec![], vec![])
        )
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::stmt::aggregate_with_argtypes;
use pg_ast::AggregateWithArgs;
use pg_lexer::Keyword::Aggregate;
