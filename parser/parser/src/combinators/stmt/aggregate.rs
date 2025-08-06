pub(super) fn aggregate(stream: &mut TokenStream) -> scan::Result<AggregateWithArgs> {

    /*
        AGGREGATE aggregate_with_argtypes
    */

    let (_, signature) = seq!(Aggregate, aggregate_with_argtypes).parse(stream)?;

    Ok(signature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_aggregate() {
        test_parser!(
            source = "aggregate foo(*)",
            parser = aggregate,
            expected = AggregateWithArgs::new(vec!["foo".into()], vec![], vec![])
        )
    }
}

use crate::combinators::stmt::aggregate_with_argtypes;
use pg_ast::AggregateWithArgs;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Aggregate;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
