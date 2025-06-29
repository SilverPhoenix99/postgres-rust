pub(in crate::combinators) fn or<T: OrCombinator>(alternatives: T) -> impl Combinator<Output = T::Output> {
    OrCombi(alternatives)
}

struct OrCombi<T>(T);

impl<T: OrCombinator> Combinator for OrCombi<T>
{
    type Output = T::Output;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        self.0.parse(stream)
    }
}

pub(in crate::combinators) trait OrCombinator {
    type Output;

    fn parse(&self, stream: &mut TokenStream) -> scan::Result<Self::Output>;
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

                fn parse(&self, stream: &mut TokenStream) -> scan::Result<Self::Output> {

                    if let Some(ok) = self.0.parse(stream).optional()? {
                        return Ok(ok)
                    }

                    $(
                        if let Some(ok) = self.$f.parse(stream).optional()? {
                            return Ok(ok)
                        }
                    )+

                    let loc = stream.current_location();
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
);

use crate::combinators::foundation::combinator::Combinator;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
