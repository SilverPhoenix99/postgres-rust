/// Alias: `opt_partition_clause`
pub(super) fn partition_clause(stream: &mut TokenStream) -> scan::Result<Vec<ExprNode>> {

    /*
        PARTITION BY expr_list
    */

    let (.., exprs) = seq!(Partition, By, expr_list)
        .parse(stream)?;

    Ok(exprs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("partition by 1, 2", vec![IntegerConst(1), IntegerConst(2)])]
    fn test_partition_clause(source: &str, expected: Vec<ExprNode>) {
        test_parser!(source, partition_clause, expected);
    }
}

use crate::combinators::expr_list;
use crate::combinators::foundation::seq;
use pg_ast::ExprNode;
use pg_combinators::Combinator;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Partition;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
