/// Alias: `opt_partition_clause`
pub(super) fn partition_clause(ctx: &mut ParserContext) -> scan::Result<Vec<ExprNode>> {

    /*
        PARTITION BY expr_list
    */

    let expr_list = ctx.expr_list();
    let (.., exprs) = seq!(Partition, By, expr_list)
        .parse(ctx)?;

    Ok(exprs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::expr_list;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("partition by 1, 2" => Ok(vec![IntegerConst(1), IntegerConst(2)]))]
    fn test_partition_clause(source: &str) -> scan::Result<Vec<ExprNode>> {
        let mut ctx = ParserContext::new(source, expr_list);
        partition_clause(&mut ctx)
    }
}

use pg_ast::ExprNode;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Partition;
use pg_parser_core::scan;
