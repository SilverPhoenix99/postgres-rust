/**
Joins multiple parsers into a single parser.
* Returns the first `Ok` in order.
* If none return `Ok`, then the parser returns `Err(NoMatch)`.
* If a parser returns a `ScanErr`, that error is returned immediately.

Equivalent to `A | B ( | ... )*`.
*/
#[cfg(not(feature = "tuple_combinators"))]
#[macro_export]
macro_rules! alt {
    (
        $head:expr,
        $(
            $tail:expr
        ),+
        $(,)?
    ) => {
        $crate::combinators::core::parser(|ctx| {

            let p = $head;
            let result = $crate::combinators::core::Combinator::parse(&p, ctx);
            let result = pg_parser_core::Optional::optional(result)?;
            if let Some(ok) = result {
                return Ok(ok)
            }

            $(
                let p = $tail;
                let result = $crate::combinators::core::Combinator::parse(&p, ctx);
                let result = pg_parser_core::Optional::optional(result)?;
                if let Some(ok) = result {
                    return Ok(ok)
                }
            )+

            Err(pg_parser_core::scan::Error::NoMatch(ctx.stream_mut().current_location()))
        })
    };
}

/**
Joins multiple parsers into a single parser.
* Returns the first `Ok` in order.
* If none return `Ok`, then the parser returns `Err(NoMatch)`.
* If a parser returns a `ScanErr`, that error is returned immediately.

Equivalent to `A | B ( | ... )*`.
*/
#[cfg(feature = "tuple_combinators")]
#[macro_export]
macro_rules! alt {
    (
        $head:expr,
        $(
            $tail:expr
        ),+
        $(,)?
    ) => {
        $crate::combinators::core::or(($head, $($tail),+))
    };
}
