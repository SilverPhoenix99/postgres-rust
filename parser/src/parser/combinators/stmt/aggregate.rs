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
    use crate::parser::tests::test_parser;

    #[test]
    fn test_aggregate() {
        test_parser!(
            source = "aggregate foo(*)",
            parser = aggregate(),
            expected = AggregateWithArgs::new(vec!["foo".into()], vec![], vec![])
        )
    }
}

use crate::lexer::Keyword::Aggregate;
use crate::parser::ast_node::AggregateWithArgs;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::stmt::aggregate_with_argtypes;
