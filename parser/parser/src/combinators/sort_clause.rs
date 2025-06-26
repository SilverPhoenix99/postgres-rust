/// Aliases:
/// * `opt_sort_clause`
/// * `json_array_aggregate_order_by_clause_opt`
pub(super) fn sort_clause(stream: &mut TokenStream) -> scan::Result<Vec<SortBy>> {

    /*
        ORDER BY sortby_list
    */

    let (.., sorts) = seq!(stream => Order, By, sortby_list)?;

    Ok(sorts)
}

fn sortby_list(stream: &mut TokenStream) -> scan::Result<Vec<SortBy>> {

    /*
        sortby ( ',' sortby )*
    */

    many!(stream => sep = Comma, sortby)
}

fn sortby(stream: &mut TokenStream) -> scan::Result<SortBy> {

    /*
          a_expr USING qual_all_Op opt_nulls_order
        | a_expr opt_asc_desc opt_nulls_order
    */

    let (expr, direction, nulls) = seq!(=>
        a_expr().parse(stream),
        choice!(stream =>
            seq!(stream => Kw::Using, qual_all_op)
                .map(|(_, op)| Some(Using(op))),
            opt_asc_desc(stream)
        )
            .optional()
            .map(Option::unwrap_or_default),
        opt_nulls_order(stream)
    )?;

    Ok(SortBy::new(expr, direction, nulls))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::Operator::Less;
    use pg_ast::SortDirection;
    use pg_ast::SortDirection::Ascending;
    use pg_ast::SortNulls::NullsFirst;
    use pg_ast::SortNulls::NullsLast;
    use test_case::test_case;

    #[test]
    fn test_sort_clause() {
        test_parser!(
            source = "order by 1, 2",
            parser = sort_clause,
            expected = vec![
                SortBy::new(IntegerConst(1), None, None),
                SortBy::new(IntegerConst(2), None, None),
            ]
        )
    }

    #[test]
    fn test_sortby_list() {
        test_parser!(
            source = "1, 2 nulls last, 3 using <, 4 asc",
            parser = sortby_list,
            expected = vec![
                SortBy::new(IntegerConst(1), None, None),
                SortBy::new(IntegerConst(2), None, Some(NullsLast)),
                SortBy::new(IntegerConst(3), Some(Using(Less.into())), None),
                SortBy::new(IntegerConst(4), Some(Ascending), None),
            ]
        )
    }

    #[test_case("1 using < nulls first", SortBy::new(
        IntegerConst(1),
        Some(Using(Less.into())),
        Some(NullsFirst)
    ))]
    #[test_case("2 asc nulls last", SortBy::new(
        IntegerConst(2),
        Some(Ascending),
        Some(NullsLast)
    ))]
    #[test_case("3 desc", SortBy::new(
        IntegerConst(3),
        Some(SortDirection::Descending),
        None
    ))]
    #[test_case("4", SortBy::new(
        IntegerConst(4),
        None,
        None
    ))]
    #[test_case("5 nulls first", SortBy::new(
        IntegerConst(5),
        None,
        Some(NullsFirst)
    ))]
    fn test_sortby(source: &str, expected: SortBy) {
        test_parser!(source, sortby, expected)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::opt_asc_desc;
use crate::combinators::opt_nulls_order;
use crate::combinators::qual_all_op;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::SortBy;
use pg_ast::SortDirection::Using;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Order;
use pg_lexer::OperatorKind::Comma;
use crate::result::Optional;
