pub(super) fn frame_extent(ctx: &mut ParserContext) -> scan::Result<FrameExtent> {

    /*
          frame_bound
        | BETWEEN frame_bound AND frame_bound
    */

    alt!(between_frame_bounds, single_frame_bound)
        .parse(ctx)
}

fn between_frame_bounds(ctx: &mut ParserContext) -> scan::Result<FrameExtent> {

    let (_, start, _, Located(end, loc)) = seq!(
        Between,
        frame_bound,
        And,
        located!(frame_bound)
    ).parse(ctx)?;

    let frame = match (start, end) {
        (UnboundedPreceding, UnboundedFollowing) => {
            FrameExtent::Unbounded {
                end: Some(PrecedingEnd::Unbounded)
            }
        },
        (UnboundedPreceding, CurrentRow) => {
            FrameExtent::Unbounded {
                end: Some(PrecedingEnd::CurrentRow)
            }
        },
        (UnboundedPreceding, OffsetPreceding(end)) => {
            FrameExtent::Unbounded {
                end: Some(PrecedingEnd::Preceding(end))
            }
        },
        (UnboundedPreceding, OffsetFollowing(end)) => {
            FrameExtent::Unbounded {
                end: Some(PrecedingEnd::Following(end))
            }
        },
        (CurrentRow, UnboundedFollowing) => {
            FrameExtent::CurrentRow {
                end: Some(CurrentRowEnd::Unbounded)
            }
        },
        (CurrentRow, CurrentRow) => {
            FrameExtent::CurrentRow {
                end: Some(CurrentRowEnd::CurrentRow)
            }
        },
        (CurrentRow, OffsetFollowing(end)) => {
            FrameExtent::CurrentRow {
                end: Some(CurrentRowEnd::Following(end))
            }
        },
        (OffsetPreceding(start), UnboundedFollowing) => {
            FrameExtent::Preceding {
                start,
                end: Some(PrecedingEnd::Unbounded)
            }
        },
        (OffsetPreceding(start), CurrentRow) => {
            FrameExtent::Preceding {
                start,
                end: Some(PrecedingEnd::CurrentRow)
            }
        },
        (OffsetPreceding(start), OffsetPreceding(end)) => {
            FrameExtent::Preceding {
                start,
                end: Some(PrecedingEnd::Preceding(end))
            }
        },
        (OffsetPreceding(start), OffsetFollowing(end)) => {
            FrameExtent::Preceding {
                start,
                end: Some(PrecedingEnd::Following(end))
            }
        },
        (OffsetFollowing(start), UnboundedFollowing) => {
            FrameExtent::Following {
                start,
                end: FollowingEnd::Unbounded
            }
        },
        (OffsetFollowing(start), OffsetFollowing(end)) => {
            FrameExtent::Following {
                start,
                end: FollowingEnd::Following(end)
            }
        },
        // Illegal combinations:
        (UnboundedFollowing, _) => {
            return Err(InvalidUnboundedFollowingFrame.at_location(loc).into())
        },
        (_, UnboundedPreceding) => {
            return Err(InvalidUnboundedPrecedingFrame.at_location(loc).into())
        },
        (CurrentRow, OffsetPreceding(_)) => {
            return Err(InvalidCurrentRowFrame.at_location(loc).into())
        },
        (OffsetFollowing(_), CurrentRow | OffsetPreceding(_)) => {
            return Err(InvalidStartFollowingEndPrecedingFrame.at_location(loc).into())
        },
    };

    Ok(frame)
}

fn single_frame_bound(ctx: &mut ParserContext) -> scan::Result<FrameExtent> {

    let Located(bound, loc) = located!(frame_bound).parse(ctx)?;

    let frame = match bound {
        UnboundedPreceding => FrameExtent::Unbounded { end: None },
        CurrentRow => FrameExtent::CurrentRow { end: None },
        OffsetPreceding(start) => FrameExtent::Preceding { start, end: None },
        // Illegal options:
        UnboundedFollowing => {
            return Err(InvalidUnboundedFollowingFrame.at_location(loc).into())
        },
        OffsetFollowing(_) => {
            return Err(InvalidOffsetFollowingFrame.at_location(loc).into())
        },
    };

    Ok(frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_matrix;

    #[test_matrix("unbounded preceding" => Ok(
        FrameExtent::Unbounded { end: None }
    ))]
    #[test_matrix("current row" => Ok(
        FrameExtent::CurrentRow { end: None }
    ))]
    #[test_matrix("1 preceding" => Ok(
        FrameExtent::Preceding {
            start: IntegerConst(1),
            end: None
        }
    ))]
    #[test_matrix("between unbounded preceding and unbounded following" => Ok(
        FrameExtent::Unbounded { end: Some(PrecedingEnd::Unbounded) }
    ))]
    #[test_matrix("between unbounded preceding and current row" => Ok(
        FrameExtent::Unbounded { end: Some(PrecedingEnd::CurrentRow) }
    ))]
    #[test_matrix("between unbounded preceding and 1 preceding" => Ok(
        FrameExtent::Unbounded { end: Some(PrecedingEnd::Preceding(IntegerConst(1))) }
    ))]
    #[test_matrix("between unbounded preceding and 1 following" => Ok(
        FrameExtent::Unbounded { end: Some(PrecedingEnd::Following(IntegerConst(1))) }
    ))]
    #[test_matrix("between current row and unbounded following" => Ok(
        FrameExtent::CurrentRow { end: Some(CurrentRowEnd::Unbounded) }
    ))]
    #[test_matrix("between current row and current row" => Ok(
        FrameExtent::CurrentRow { end: Some(CurrentRowEnd::CurrentRow) }
    ))]
    #[test_matrix("between current row and 1 following" => Ok(
        FrameExtent::CurrentRow { end: Some(CurrentRowEnd::Following(IntegerConst(1))) }
    ))]
    #[test_matrix("between 1 preceding and unbounded following" => Ok(
        FrameExtent::Preceding {
            start: IntegerConst(1),
            end: Some(PrecedingEnd::Unbounded)
        }
    ))]
    #[test_matrix("between 1 preceding and current row" => Ok(
        FrameExtent::Preceding {
            start: IntegerConst(1),
            end: Some(PrecedingEnd::CurrentRow)
        }
    ))]
    #[test_matrix("between 1 preceding and 1 preceding" => Ok(
        FrameExtent::Preceding {
            start: IntegerConst(1),
            end: Some(PrecedingEnd::Preceding(IntegerConst(1)))
        }
    ))]
    #[test_matrix("between 1 preceding and 1 following" => Ok(
        FrameExtent::Preceding {
            start: IntegerConst(1),
            end: Some(PrecedingEnd::Following(IntegerConst(1)))
        }
    ))]
    #[test_matrix("between 1 following and unbounded following" => Ok(
        FrameExtent::Following {
            start: IntegerConst(1),
            end: FollowingEnd::Unbounded
        }
    ))]
    #[test_matrix("between 1 following and 1 following" => Ok(
        FrameExtent::Following {
            start: IntegerConst(1),
            end: FollowingEnd::Following(IntegerConst(1))
        }
    ))]
    fn test_frame_extent(source: &str) -> scan::Result<FrameExtent> {
        test_parser!(source, frame_extent)
    }
}

use super::frame_bound::frame_bound;
use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::window_specification::frame_bound::FrameBound::CurrentRow;
use crate::combinators::window_specification::frame_bound::FrameBound::OffsetFollowing;
use crate::combinators::window_specification::frame_bound::FrameBound::OffsetPreceding;
use crate::combinators::window_specification::frame_bound::FrameBound::UnboundedFollowing;
use crate::combinators::window_specification::frame_bound::FrameBound::UnboundedPreceding;
use crate::located;
use crate::seq;
use crate::ParserContext;
use pg_ast::CurrentRowEnd;
use pg_ast::FollowingEnd;
use pg_ast::FrameExtent;
use pg_ast::PrecedingEnd;
use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_elog::parser::Error::InvalidCurrentRowFrame;
use pg_elog::parser::Error::InvalidOffsetFollowingFrame;
use pg_elog::parser::Error::InvalidStartFollowingEndPrecedingFrame;
use pg_elog::parser::Error::InvalidUnboundedFollowingFrame;
use pg_elog::parser::Error::InvalidUnboundedPrecedingFrame;
use pg_lexer::Keyword::And;
use pg_lexer::Keyword::Between;
use pg_parser_core::scan;
