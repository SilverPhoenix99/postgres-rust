pub(super) fn opt_frame_clause() -> impl Combinator<Output = Option<WindowFrame>> {

    /*
        RANGE frame_extent opt_window_exclusion_clause
      | ROWS frame_extent opt_window_exclusion_clause
      | GROUPS frame_extent opt_window_exclusion_clause
    */

    sequence! {
        match_first! {
            RangeKw.map(|_| Range),
            Kw::Rows.map(|_| Rows),
            Kw::Groups.map(|_| Groups),
        },
        frame_extent(),
        opt_window_exclusion_clause()
    }
        .map(|(kind, extent, exclusion)|
            WindowFrame::new(kind, extent, exclusion)
        )
        .optional()
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
        WindowExclusion::Ties,
    };
    use test_case::test_case;

    #[test_case("range between current row and unbounded following", Some(
        WindowFrame::new(
            Range,
            FrameExtent::CurrentRow { end: Some(CurrentRowEnd::Unbounded) },
            None
        )
    ))]
    #[test_case("rows current row exclude ties", Some(
        WindowFrame::new(
            Rows,
            FrameExtent::CurrentRow { end: None },
            Some(Ties)
        )
    ))]
    #[test_case("groups unbounded preceding", Some(
        WindowFrame::new(
            Groups,
            FrameExtent::Unbounded { end: None },
            None
        )
    ))]
    #[test_case("something else", None)]
    #[test_case("", None)]
    fn test_opt_frame_clause(source: &str, expected: Option<WindowFrame>) {
        test_parser!(source, opt_frame_clause(), expected);
    }
}

use super::frame_extent::frame_extent;
use super::opt_window_exclusion_clause::opt_window_exclusion_clause;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::WindowFrame;
use pg_ast::WindowFrameKind::Groups;
use pg_ast::WindowFrameKind::Range;
use pg_ast::WindowFrameKind::Rows;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::RangeKw;
