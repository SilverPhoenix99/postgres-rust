pub(super) fn frame_extent(stream: &mut TokenStream<'_>) -> scan::Result<FrameExtent> {

    /*
          frame_bound
        | BETWEEN frame_bound AND frame_bound
    */

    alt!(between_frame_bounds, single_frame_bound)
        .parse(stream)
}

fn between_frame_bounds(stream: &mut TokenStream<'_>) -> scan::Result<FrameExtent> {

    let (_, start, _, (end, loc)) = seq!(
        Between,
        frame_bound,
        And,
        located!(frame_bound)
    ).parse(stream)?;

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
            let err = InvalidUnboundedFollowingFrame.at(loc);
            return Err(err.into())
        },
        (_, UnboundedPreceding) => {
            let err = InvalidUnboundedPrecedingFrame.at(loc);
            return Err(err.into())
        },
        (CurrentRow, OffsetPreceding(_)) => {
            let err = InvalidCurrentRowFrame.at(loc);
            return Err(err.into())
        },
        (OffsetFollowing(_), CurrentRow | OffsetPreceding(_)) => {
            let err = InvalidStartFollowingEndPrecedingFrame.at(loc);
            return Err(err.into())
        },
    };

    Ok(frame)
}

fn single_frame_bound(stream: &mut TokenStream<'_>) -> scan::Result<FrameExtent> {

    let (bound, loc) = located!(frame_bound).parse(stream)?;

    let frame = match bound {
        UnboundedPreceding => FrameExtent::Unbounded { end: None },
        CurrentRow => FrameExtent::CurrentRow { end: None },
        OffsetPreceding(start) => FrameExtent::Preceding { start, end: None },
        // Illegal options:
        UnboundedFollowing => {
            let err = InvalidUnboundedFollowingFrame.at(loc);
            return Err(err.into())
        },
        OffsetFollowing(_) => {
            let err = InvalidOffsetFollowingFrame.at(loc);
            return Err(err.into())
        },
    };

    Ok(frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("unbounded preceding",
        FrameExtent::Unbounded { end: None }
    )]
    #[test_case("current row",
        FrameExtent::CurrentRow { end: None }
    )]
    #[test_case("1 preceding",
        FrameExtent::Preceding {
            start: IntegerConst(1),
            end: None
        }
    )]
    #[test_case("between unbounded preceding and unbounded following",
        FrameExtent::Unbounded { end: Some(PrecedingEnd::Unbounded) }
    )]
    #[test_case("between unbounded preceding and current row",
        FrameExtent::Unbounded { end: Some(PrecedingEnd::CurrentRow) }
    )]
    #[test_case("between unbounded preceding and 1 preceding",
        FrameExtent::Unbounded { end: Some(PrecedingEnd::Preceding(IntegerConst(1))) }
    )]
    #[test_case("between unbounded preceding and 1 following",
        FrameExtent::Unbounded { end: Some(PrecedingEnd::Following(IntegerConst(1))) }
    )]
    #[test_case("between current row and unbounded following",
        FrameExtent::CurrentRow { end: Some(CurrentRowEnd::Unbounded) }
    )]
    #[test_case("between current row and current row",
        FrameExtent::CurrentRow { end: Some(CurrentRowEnd::CurrentRow) }
    )]
    #[test_case("between current row and 1 following",
        FrameExtent::CurrentRow { end: Some(CurrentRowEnd::Following(IntegerConst(1))) }
    )]
    #[test_case("between 1 preceding and unbounded following",
        FrameExtent::Preceding {
            start: IntegerConst(1),
            end: Some(PrecedingEnd::Unbounded)
        }
    )]
    #[test_case("between 1 preceding and current row",
        FrameExtent::Preceding {
            start: IntegerConst(1),
            end: Some(PrecedingEnd::CurrentRow)
        }
    )]
    #[test_case("between 1 preceding and 1 preceding",
        FrameExtent::Preceding {
            start: IntegerConst(1),
            end: Some(PrecedingEnd::Preceding(IntegerConst(1)))
        }
    )]
    #[test_case("between 1 preceding and 1 following",
        FrameExtent::Preceding {
            start: IntegerConst(1),
            end: Some(PrecedingEnd::Following(IntegerConst(1)))
        }
    )]
    #[test_case("between 1 following and unbounded following",
        FrameExtent::Following {
            start: IntegerConst(1),
            end: FollowingEnd::Unbounded
        }
    )]
    #[test_case("between 1 following and 1 following",
        FrameExtent::Following {
            start: IntegerConst(1),
            end: FollowingEnd::Following(IntegerConst(1))
        }
    )]
    fn test_frame_extent(source: &str, expected: FrameExtent) {
        test_parser!(source, frame_extent, expected)
    }
}

use super::frame_bound::frame_bound;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::located;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::window_specification::frame_bound::FrameBound::CurrentRow;
use crate::combinators::window_specification::frame_bound::FrameBound::OffsetFollowing;
use crate::combinators::window_specification::frame_bound::FrameBound::OffsetPreceding;
use crate::combinators::window_specification::frame_bound::FrameBound::UnboundedFollowing;
use crate::combinators::window_specification::frame_bound::FrameBound::UnboundedPreceding;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::CurrentRowEnd;
use pg_ast::FollowingEnd;
use pg_ast::FrameExtent;
use pg_ast::PrecedingEnd;
use pg_elog::parser::Error::InvalidCurrentRowFrame;
use pg_elog::parser::Error::InvalidOffsetFollowingFrame;
use pg_elog::parser::Error::InvalidStartFollowingEndPrecedingFrame;
use pg_elog::parser::Error::InvalidUnboundedFollowingFrame;
use pg_elog::parser::Error::InvalidUnboundedPrecedingFrame;
use pg_lexer::Keyword::And;
use pg_lexer::Keyword::Between;
