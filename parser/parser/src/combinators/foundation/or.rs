/// Joins multiple parsers into a single parser,
/// and where the returned parser returns the first `Ok`.
///
/// Equivalent to `A | B ( | ... )*`.
macro_rules! match_first {

    ($head:expr , $($tail:expr),+ $(,)?) => {
        $crate::combinators::foundation::parser(|stream| {
            #[allow(unused_imports)]
            use $crate::combinators::foundation::Combinator;
            use $crate::scan::Error::ScanErr;

            let p = $head;
            let result = p.parse(stream);
            if let Ok(_) | Err(ScanErr(_)) = result {
                return result
            }

            $(
                let p = $tail;
                let result = p.parse(stream);
                if let Ok(_) | Err(ScanErr(_)) = result {
                    return result
                }
            )+

            result
        })
    };
}

pub(in crate::combinators) use match_first;

/// Pair this with [`chain()`](super::CombinatorHelpers::chain).
macro_rules! match_first_with_state {
    (
        $($move:ident)? |$state:pat, $stream:ident| {
            $head:expr => ($head_pat:pat) $head_expr:expr,
            $( $tail:expr => ($tail_pat:pat) $tail_expr:expr ),+
            $(,)?
        }
    ) => {
        $($move)? |$state, $stream| {
            use $crate::combinators::foundation::Combinator;
            use $crate::scan::Error::{Eof, NoMatch, ScanErr};

            let p = $head;
            let result = p.parse($stream);
            match result {
                Ok($head_pat) => return Ok($head_expr),
                Err(ScanErr(err)) => return Err(ScanErr(err)),
                _ => {}
            }

            $(
                let p = $tail;
                let result = p.parse($stream);
                match result {
                    Ok($tail_pat) => return Ok($tail_expr),
                    Err(ScanErr(err)) => return Err(ScanErr(err)),
                    _ => {}
                }
            )+

            match result {
                Err(NoMatch(loc)) => Err(NoMatch(loc)),
                Err(Eof(loc)) => Err(NoMatch(loc)),
                // SAFETY: Ok and ScanErr are matched above
                _ => unsafe { core::hint::unreachable_unchecked() }
            }
        }
    };
}

pub(in crate::combinators) use match_first_with_state;

/// Returns the first `Ok` result between the 2 parsers.
///
/// This is equivalent to `L | R`.
pub(in crate::combinators) fn or<L, R>(left: L, right: R)
    -> OrCombi<L, R>
where
    L: Combinator,
    R: Combinator<Output = L::Output>,
{
    OrCombi { left, right }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct OrCombi<L, R> {
    left: L,
    right: R,
}

impl<L, R> Combinator for OrCombi<L, R>
where
    L: Combinator,
    R: Combinator<Output = L::Output>
{
    type Output = L::Output;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output> {

        if let Some(ok) = self.left.parse(stream).optional()? {
            return Ok(ok)
        }

        self.right.parse(stream)
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::scan::Result;
use crate::stream::TokenStream;
