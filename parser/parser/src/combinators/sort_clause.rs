/// Aliases:
/// * `opt_sort_clause`
/// * `json_array_aggregate_order_by_clause_opt`
pub(super) fn sort_clause(stream: &mut TokenStream) -> scan::Result<Located<Vec<SortBy>>> {

    /*
        ORDER BY sortby_list
    */

    let ((.., sorts), loc) = located(seq!(Order, By, sortby_list))
        .parse(stream)?;

    Ok((sorts, loc))
}

fn sortby_list(stream: &mut TokenStream) -> scan::Result<Vec<SortBy>> {

    /*
        sortby ( ',' sortby )*
    */

    many_sep(Comma, sortby).parse(stream)
}

fn sortby(stream: &mut TokenStream) -> scan::Result<SortBy> {

    /*
          a_expr USING qual_all_Op ( nulls_order )?
        | a_expr ( asc_desc )? ( nulls_order )?
    */

    let (expr, direction, nulls) = seq!(
        a_expr,
        alt!(
            seq!(Kw::Using, qual_all_op)
                .map(|(_, op)| Some(Using(op))),
            asc_desc.optional()
        )
            .optional()
            .map(Option::unwrap_or_default),
        nulls_order.optional()
    ).parse(stream)?;

    Ok(SortBy::new(expr, direction, nulls))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::stream;
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
        let mut stream = stream("order by 1, 2");

        let (actual, _) = sort_clause(&mut stream).unwrap();

        let expected = vec![
            SortBy::new(IntegerConst(1), None, None),
            SortBy::new(IntegerConst(2), None, None),
        ];

        assert_eq!(expected, actual);
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

use crate::combinators::asc_desc;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::nulls_order;
use crate::combinators::qual_all_op;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::SortBy;
use pg_ast::SortDirection::Using;
use pg_basics::Located;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Order;
use pg_lexer::OperatorKind::Comma;
