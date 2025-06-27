pub(super) fn opt_partition_clause(stream: &mut TokenStream) -> scan::Result<Option<Vec<ExprNode>>> {

    /*
        PARTITION BY expr_list
    */

    let exprs = seq!(stream => Partition, By, expr_list)
        .optional()?
        .map(|(.., exprs)| exprs);

    Ok(exprs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("partition by 1, 2", Some(vec![IntegerConst(1), IntegerConst(2)]))]
    #[test_case("something else", None)]
    #[test_case("", None)]
    fn test_opt_partition_clause(source: &str, expected: Option<Vec<ExprNode>>) {
        test_parser!(source, opt_partition_clause, expected);
    }
}

use crate::combinators::expr_list;
use crate::combinators::foundation::seq;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Partition;
