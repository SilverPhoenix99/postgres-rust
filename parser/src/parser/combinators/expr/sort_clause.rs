fn sortby() -> impl Combinator<Output = SortBy> {

    /*
          a_expr USING qual_all_Op opt_nulls_order
        | a_expr opt_asc_desc opt_nulls_order
    */

    sequence!(
        a_expr(),
        or(
            Kw::Using
                .and_then(qual_all_op(),
                    |_, op| Some(Using(op))
                ),
            opt_asc_desc()
        ),
        opt_nulls_order()
    )
        .map(|(expr, direction, nulls)|
            SortBy::new(Box::new(expr), direction, nulls)
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::{
        ExprNode::IntegerConst,
        Operator::Less,
        SortDirection,
        SortNulls,
    };
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test_case("1 using < nulls first", SortBy::new(
        Box::new(IntegerConst(1)),
        Some(Using(Less.into())),
        Some(SortNulls::First)
    ))]
    #[test_case("2 asc nulls last", SortBy::new(
        Box::new(IntegerConst(2)),
        Some(SortDirection::Ascending),
        Some(SortNulls::Last)
    ))]
    #[test_case("3 desc", SortBy::new(
        Box::new(IntegerConst(3)),
        Some(SortDirection::Descending),
        None
    ))]
    #[test_case("4", SortBy::new(
        Box::new(IntegerConst(4)),
        None,
        None
    ))]
    fn test_sortby(source: &str, expected: SortBy) {
        test_parser!(source, sortby(), expected)
    }
}

use crate::lexer::Keyword as Kw;
use crate::parser::ast_node::SortBy;
use crate::parser::ast_node::SortDirection::Using;
use crate::parser::combinators::expr::a_expr;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::opt_asc_desc;
use crate::parser::combinators::opt_nulls_order;
use crate::parser::combinators::qual_all_op;
