/// Post-condition: Vec **May** be empty.
pub(super) fn opt_partition_clause() -> impl Combinator<Output = Vec<ExprNode>> {

    /*
        PARTITION BY expr_list
    */

    and(Partition, By)
        .and_right(expr_list())
        .optional()
        .map(Option::unwrap_or_default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("partition by 1, 2", vec![IntegerConst(1), IntegerConst(2)])]
    #[test_case("something else", vec![])]
    #[test_case("", vec![])]
    fn test_opt_partition_clause(source: &str, expected: Vec<ExprNode>) {
        test_parser!(source, opt_partition_clause(), expected);
    }
}

use crate::combinators::expr_list;
use crate::combinators::foundation::and;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::ExprNode;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Partition;
