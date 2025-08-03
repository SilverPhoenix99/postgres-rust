/// Joins multiple parsers into a single parser.
/// * Returns the first `Ok` in order.
/// * If none return `Ok`, then the parser returns `Err(NoMatch)`.
/// * If a parser returns a `ScanErr`, that error is returned immediately.
///
/// Equivalent to `A | B ( | ... )*`.
macro_rules! alt {
    (
        $head:expr,
        $(
            $tail:expr
        ),+
        $(,)?
    ) => {
        $crate::combinators::foundation::parser(move |stream| {
            #[allow(unused_imports)]
            use $crate::{
                combinators::foundation::Combinator,
                result::Optional,
                scan::Error::NoMatch,
                no_match
            };

            if let Some(ok) = $head.parse(stream).optional()? {
                return Ok(ok)
            }

            $(
                if let Some(ok) = $tail.parse(stream).optional()? {
                    return Ok(ok)
                }
            )+

            no_match(stream)
        })
    };
}

pub(in crate::combinators) use alt;
