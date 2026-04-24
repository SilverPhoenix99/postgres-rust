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
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_matrix;

    #[test_matrix("partition by 1, 2" => Ok(vec![IntegerConst(1), IntegerConst(2)]))]
    fn test_partition_clause(source: &str) -> scan::Result<Vec<ExprNode>> {
        test_parser!(source, partition_clause)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::expr_list;
use crate::seq;
use crate::ParserContext;
use pg_ast::ExprNode;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Partition;
use pg_parser_core::scan;
