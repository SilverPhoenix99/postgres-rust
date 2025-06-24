pub(super) fn aggregate(stream: &mut TokenStream) -> Result<AggregateWithArgs> {

    /*
        AGGREGATE aggregate_with_argtypes
    */

    let (_, signature) = seq!(stream => Aggregate, aggregate_with_argtypes)?;

    Ok(signature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_aggregate() {
        test_parser!(
            source = "aggregate foo(*)",
            parser = aggregate,
            expected = AggregateWithArgs::new(vec!["foo".into()], vec![], vec![])
        )
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::stmt::aggregate_with_argtypes;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::AggregateWithArgs;
use pg_lexer::Keyword::Aggregate;
