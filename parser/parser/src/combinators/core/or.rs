#![cfg(feature = "tuple_combinators")]

pub(in crate::combinators) fn or<T: OrCombinator>(alternatives: T) -> impl Combinator<Output = T::Output> {
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

pub(in crate::combinators) trait OrCombinator {
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

    (@impl $($t:ident : $f:tt),+) => {

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
    };

    (
        [($t_last:ident : $f_last:tt)], [$(($t_prefix:ident : $f_prefix:tt)),+]
    ) => {
        tuple_or_combinator! { @impl $($t_prefix : $f_prefix),+ , $t_last : $f_last }
    };

    (
        [($t_next:ident : $f_next:tt) , $(($t_suffix:ident : $f_suffix:tt)),+], []
    ) => {
        tuple_or_combinator! { @impl $t_next : $f_next }

        tuple_or_combinator! {
            [$(($t_suffix : $f_suffix)),+],
            [($t_next : $f_next)]
        }
    };

    (
        [($t_next:ident : $f_next:tt) , $(($t_suffix:ident : $f_suffix:tt)),+], [$(($t_prefix:ident : $f_prefix:tt)),+]
    ) => {
        tuple_or_combinator! { @impl $($t_prefix : $f_prefix),+ , $t_next : $f_next }

        tuple_or_combinator! {
            [$(($t_suffix : $f_suffix)),+],
            [$(($t_prefix : $f_prefix)),+ , ($t_next : $f_next)]
        }
    };

    ($($t:ident : $f:tt),+ $(,)?) => {
        tuple_or_combinator! { [$(($t : $f)),+], [] }
    };
}

tuple_or_combinator! {
     T1: 1,  T2: 2,  T3: 3,  T4: 4,  T5: 5,  T6: 6,  T7: 7,  T8: 8,  T9: 9, T10:10,
    T11:11, T12:12, T13:13, T14:14, T15:15, T16:16, T17:17, T18:18, T19:19, T20:20,
    T21:21, T22:22, T23:23, T24:24, T25:25, T26:26, T27:27, T28:28, T29:29, T30:30,
    T31:31, T32:32, T33:33, T34:34, T35:35, T36:36,
}

use crate::combinators::core::Combinator;
use crate::ParserContext;
use pg_parser_core::scan;
use pg_parser_core::Optional;
