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

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {

        if let Some(ok) = self.left.parse(stream).optional()? {
            return Ok(ok)
        }

        self.right.parse(stream)
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
