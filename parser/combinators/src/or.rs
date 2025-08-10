#![cfg(feature = "tuple_combinators")]

pub fn or<T: OrCombinator>(alternatives: T) -> impl Combinator<Output = T::Output> {
    OrCombi(alternatives)
}

struct OrCombi<T>(T);

impl<T: OrCombinator> Combinator for OrCombi<T>
{
    type Output = T::Output;

    fn parse(&self, ctx: &mut ParserContext<'_>) -> scan::Result<Self::Output> {
        self.0.parse(ctx)
    }
}

pub trait OrCombinator {
    type Output;

    fn parse(&self, ctx: &mut ParserContext) -> scan::Result<Self::Output>;
}

/// Joins multiple parsers into a single parser.
/// * Returns the first `Ok` in order.
/// * If none return `Ok`, then the parser returns `Err(NoMatch)`.
/// * If a parser returns a `ScanErr`, that error is returned immediately.
///
/// Equivalent to `A | B ( | ... )*`.
macro_rules! tuple_or_combinator {
    (
        $(
            ($($t:ident : $f:tt),+ $(,)? )
        );+
        $(;)?
    ) => {

        $(
            impl<T0, $($t),+> OrCombinator for (T0, $($t),+)
            where
                T0: Combinator,
                $($t: Combinator<Output = T0::Output>),+
            {
                type Output = T0::Output;

                fn parse(&self, ctx: &mut ParserContext) -> scan::Result<Self::Output> {

                    if let Some(ok) = self.0.parse(ctx).optional()? {
                        return Ok(ok)
                    }

                    $(
                        if let Some(ok) = self.$f.parse(ctx).optional()? {
                            return Ok(ok)
                        }
                    )+

                    let loc = ctx.stream_mut().current_location();
                    Err(scan::Error::NoMatch(loc))
                }
            }
        )+
    };
}

tuple_or_combinator!(
    (T1:1);
    (T1:1, T2:2);
    (T1:1, T2:2, T3:3);
    (T1:1, T2:2, T3:3, T4:4);
    (T1:1, T2:2, T3:3, T4:4, T5:5);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15, T16:16);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15, T16:16, T17:17);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15, T16:16, T17:17, T18:18);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15, T16:16, T17:17, T18:18, T19:19);
);

use crate::Combinator;
use pg_parser_core::scan;
use pg_parser_core::Optional;
use pg_parser_core::ParserContext;
