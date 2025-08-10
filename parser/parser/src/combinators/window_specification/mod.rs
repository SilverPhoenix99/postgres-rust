pg_basics::reexport! {
    existing_window_name,
    frame_bound,
    frame_clause,
    frame_extent,
    partition_clause,
    window_exclusion_clause,
}

pub(super) fn window_specification(ctx: &mut ParserContext) -> scan::Result<WindowDefinition> {

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
    )).parse(ctx)?;

    let order = order.map(|Located(order, _)| order);

    let expr = WindowDefinition::new(name, partition, order, frame);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::expr_list;
    use pg_ast::ExprNode;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::FrameExtent;
    use pg_ast::SortBy;
    use pg_ast::WindowExclusion;
    use pg_ast::WindowFrame;
    use pg_ast::WindowFrameKind;
    use test_case::test_case;

    #[test_case("(foo partition by 1 order by 2 range current row)" => Ok(WindowDefinition::new(Some("foo".into()), some_partition(), some_order(), some_frame())))]
    #[test_case("(foo partition by 1 order by 2)"                   => Ok(WindowDefinition::new(Some("foo".into()), some_partition(), some_order(), None)))]
    #[test_case("(foo partition by 1 range current row)"            => Ok(WindowDefinition::new(Some("foo".into()), some_partition(), None,         some_frame())))]
    #[test_case("(foo partition by 1)"                              => Ok(WindowDefinition::new(Some("foo".into()), some_partition(), None,         None)))]
    #[test_case("(foo order by 2 range current row)"                => Ok(WindowDefinition::new(Some("foo".into()), None,             some_order(), some_frame())))]
    #[test_case("(foo order by 2)"                                  => Ok(WindowDefinition::new(Some("foo".into()), None,             some_order(), None)))]
    #[test_case("(foo range current row)"                           => Ok(WindowDefinition::new(Some("foo".into()), None,             None,         some_frame())))]
    #[test_case("(foo)"                                             => Ok(WindowDefinition::new(Some("foo".into()), None,             None,         None)))]
    #[test_case("(partition by 1 order by 2 range current row)"     => Ok(WindowDefinition::new(None,               some_partition(), some_order(), some_frame())))]
    #[test_case("(partition by 1 order by 2)"                       => Ok(WindowDefinition::new(None,               some_partition(), some_order(), None)))]
    #[test_case("(partition by 1 range current row)"                => Ok(WindowDefinition::new(None,               some_partition(), None,         some_frame())))]
    #[test_case("(partition by 1)"                                  => Ok(WindowDefinition::new(None,               some_partition(), None,         None)))]
    #[test_case("(order by 2 range current row)"                    => Ok(WindowDefinition::new(None,               None,             some_order(), some_frame())))]
    #[test_case("(order by 2)"                                      => Ok(WindowDefinition::new(None,               None,             some_order(), None)))]
    #[test_case("(range current row)"                               => Ok(WindowDefinition::new(None,               None,             None,         some_frame())))]
    #[test_case("()"                                                => Ok(WindowDefinition::new(None,               None,             None,         None)))]
    fn test_window_specification(source: &str) -> scan::Result<WindowDefinition> {
        let mut ctx = ParserContext::new(source, expr_list);
        window_specification(&mut ctx)
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

use crate::combinators::sort_clause;
use pg_ast::WindowDefinition;
use pg_basics::Located;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_parser_core::scan;
