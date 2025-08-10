/// Alias: `opt_partition_clause`
pub(super) fn partition_clause(ctx: &mut ParserContext) -> scan::Result<Vec<ExprNode>> {

    /*
        PARTITION BY expr_list
    */

    let (.., exprs) = seq!(Partition, By, expr_list)
        .parse(ctx)?;

    Ok(exprs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("partition by 1, 2", vec![IntegerConst(1), IntegerConst(2)])]
    fn test_partition_clause(source: &str, expected: Vec<ExprNode>) {
        test_parser!(source, partition_clause, expected);
    }
}

use crate::combinators::expr_list;
use pg_ast::ExprNode;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Partition;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
