/// Alias: `opt_frame_clause`
pub(super) fn frame_clause(ctx: &mut ParserContext) -> scan::Result<WindowFrame> {

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
    ).parse(ctx)?;

    let frame = WindowFrame::new(kind, extent, exclusion);

    Ok(frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::CurrentRowEnd;
    use pg_ast::FrameExtent;
    use pg_ast::WindowExclusion::NoOthers;
    use pg_ast::WindowExclusion::Ties;
    use test_case::test_matrix;

    #[test_matrix("range between current row and unbounded following" => Ok(
        WindowFrame::new(
            Range,
            FrameExtent::CurrentRow { end: Some(CurrentRowEnd::Unbounded) },
            NoOthers
        )
    ))]
    #[test_matrix("rows current row exclude ties" => Ok(
        WindowFrame::new(
            Rows,
            FrameExtent::CurrentRow { end: None },
            Ties
        )
    ))]
    #[test_matrix("groups unbounded preceding" => Ok(
        WindowFrame::new(
            Groups,
            FrameExtent::Unbounded { end: None },
            NoOthers
        )
    ))]
    fn test_frame_clause(source: &str) -> scan::Result<WindowFrame> {
        test_parser!(source, frame_clause)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::window_specification::frame_extent;
use crate::combinators::window_specification::window_exclusion_clause;
use crate::seq;
use crate::ParserContext;
use pg_ast::WindowFrame;
use pg_ast::WindowFrameKind::Groups;
use pg_ast::WindowFrameKind::Range;
use pg_ast::WindowFrameKind::Rows;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::RangeKw;
use pg_parser_core::scan;
