/// Joins multiple parsers into a single parser,
/// and where the returned parser returns the first `Err`.
///
/// If all parsers return `Ok`, then a tuple with all results is returned.
///
/// Equivalent to `A & B ( & ... )*`.
macro_rules! sequence {
    ($head:expr , $($tail:expr),+ $(,)?) => {
        $crate::parser::combinators::parser(|stream| {
            Ok((
                $head.parse(stream)?,
                $({
                    $tail.required().parse(stream)?
                }),+
            ))
        })
    };
}
pub(in crate::parser) use sequence;

/// Returns the result from both parsers, in order, or the first `Err`.
///
/// This is equivalent to `L & R`.
pub(in crate::parser) fn and<L, R>(left: L, right: R) -> AndCombi<L, R>
where
    L: Combinator,
    R: Combinator
{
    AndCombi { left, right }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct AndCombi<L, R> {
    left: L,
    right: R,
}

impl<L, R> Combinator for AndCombi<L, R>
where
    L: Combinator,
    R: Combinator
{
    type Output = (L::Output, R::Output);

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {

        let first = self.left.parse(stream)?;
        let second = self.right.parse(stream).required()?;
        Ok((first, second))
    }
}

use crate::parser::combinators::Combinator;
use crate::parser::result::{Required, ScanResult};
use crate::parser::token_stream::TokenStream;
