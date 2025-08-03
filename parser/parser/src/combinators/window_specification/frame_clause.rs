/// Alias: `opt_frame_clause`
pub(super) fn frame_clause(stream: &mut TokenStream<'_>) -> scan::Result<WindowFrame> {

    /*
        RANGE frame_extent  ( window_exclusion_clause )?
      | ROWS frame_extent   ( window_exclusion_clause )?
      | GROUPS frame_extent ( window_exclusion_clause )?
    */

    let (kind, extent, exclusion) = seq!(
        alt!(
            RangeKw.map(|_| Range),
            Kw::Rows.map(|_| Rows),
            Kw::Groups.map(|_| Groups),
        ),
        frame_extent,
        window_exclusion_clause.optional()
            .map(Option::unwrap_or_default),
    ).parse(stream)?;

    let frame = WindowFrame::new(kind, extent, exclusion);

    Ok(frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        CurrentRowEnd,
        ExprNode::IntegerConst,
        FrameExtent,
        WindowExclusion::{NoOthers, Ties},
    };
    use test_case::test_case;

    #[test_case("range between current row and unbounded following",
        WindowFrame::new(
            Range,
            FrameExtent::CurrentRow { end: Some(CurrentRowEnd::Unbounded) },
            NoOthers
        )
    )]
    #[test_case("rows current row exclude ties",
        WindowFrame::new(
            Rows,
            FrameExtent::CurrentRow { end: None },
            Ties
        )
    )]
    #[test_case("groups unbounded preceding", 
        WindowFrame::new(
            Groups,
            FrameExtent::Unbounded { end: None },
            NoOthers
        )
    )]
    fn test_frame_clause(source: &str, expected: WindowFrame) {
        test_parser!(source, frame_clause, expected);
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::window_specification::frame_extent;
use crate::combinators::window_specification::window_exclusion_clause;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::WindowFrame;
use pg_ast::WindowFrameKind::Groups;
use pg_ast::WindowFrameKind::Range;
use pg_ast::WindowFrameKind::Rows;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::RangeKw;
