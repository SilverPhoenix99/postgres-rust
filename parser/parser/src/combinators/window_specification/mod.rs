pg_basics::reexport! {
    existing_window_name,
    frame_bound,
    frame_clause,
    frame_extent,
    partition_clause,
    window_exclusion_clause,
}

pub(super) fn window_specification(stream: &mut TokenStream) -> scan::Result<WindowDefinition> {

    /*
        '('
            ( existing_window_name )?
            ( partition_clause )?
            ( sort_clause )?
            ( frame_clause )?
        ')'
    */

    let (name, partition, order, frame) = paren!(seq!(
        existing_window_name.optional(),
        partition_clause.optional(),
        sort_clause.optional(),
        frame_clause.optional()
    )).parse(stream)?;

    let order = order.map(|(order, _)| order);

    let expr = WindowDefinition::new(name, partition, order, frame);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::ExprNode;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::FrameExtent;
    use pg_ast::SortBy;
    use pg_ast::WindowExclusion;
    use pg_ast::WindowFrame;
    use pg_ast::WindowFrameKind;
    use pg_basics::Str;
    use test_case::test_case;

    #[test_case("(foo partition by 1 order by 2 range current row)", Some("foo".into()), some_partition(), some_order(), some_frame())]
    #[test_case("(foo partition by 1 order by 2)",                   Some("foo".into()), some_partition(), some_order(), None)]
    #[test_case("(foo partition by 1 range current row)",            Some("foo".into()), some_partition(), None,         some_frame())]
    #[test_case("(foo partition by 1)",                              Some("foo".into()), some_partition(), None,         None)]
    #[test_case("(foo order by 2 range current row)",                Some("foo".into()), None,             some_order(), some_frame())]
    #[test_case("(foo order by 2)",                                  Some("foo".into()), None,             some_order(), None)]
    #[test_case("(foo range current row)",                           Some("foo".into()), None,             None,         some_frame())]
    #[test_case("(foo)",                                             Some("foo".into()), None,             None,         None)]
    #[test_case("(partition by 1 order by 2 range current row)",     None,               some_partition(), some_order(), some_frame())]
    #[test_case("(partition by 1 order by 2)",                       None,               some_partition(), some_order(), None)]
    #[test_case("(partition by 1 range current row)",                None,               some_partition(), None,         some_frame())]
    #[test_case("(partition by 1)",                                  None,               some_partition(), None,         None)]
    #[test_case("(order by 2 range current row)",                    None,               None,             some_order(), some_frame())]
    #[test_case("(order by 2)",                                      None,               None,             some_order(), None)]
    #[test_case("(range current row)",                               None,               None,             None,         some_frame())]
    #[test_case("()",                                                None,               None,             None,         None)]
    fn test_window_specification(
        source: &str,
        name: Option<Str>,
        partition: Option<Vec<ExprNode>>,
        order: Option<Vec<SortBy>>,
        frame: Option<WindowFrame>
    ) {
        test_parser!(
            source,
            window_specification,
            WindowDefinition::new(name, partition, order, frame)
        )
    }

    fn some_partition() -> Option<Vec<ExprNode>> {
        Some(vec![IntegerConst(1)])
    }

    fn some_order() -> Option<Vec<SortBy>> {
        Some(vec![SortBy::new(IntegerConst(2), None, None)])
    }

    fn some_frame() -> Option<WindowFrame> {
        Some(WindowFrame::new(
            WindowFrameKind::Range,
            FrameExtent::CurrentRow { end: None },
            WindowExclusion::default()
        ))
    }
}

use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::sort_clause;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::WindowDefinition;
