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
        $crate::combinators::foundation::parser(|stream| {

            let p = $head;
            let result = $crate::combinators::foundation::Combinator::parse(&p, stream);
            let result = $crate::result::Optional::optional(result)?;
            if let Some(ok) = result {
                return Ok(ok)
            }

            $(
                let p = $tail;
                let result = $crate::combinators::foundation::Combinator::parse(&p, stream);
                let result = $crate::result::Optional::optional(result)?;
                if let Some(ok) = result {
                    return Ok(ok)
                }
            )+

            $crate::no_match(stream)
        })
    };
}

pub(in crate::combinators) use alt;
