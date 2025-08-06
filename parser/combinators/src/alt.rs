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
        pg_combinators::parser(|stream| {

            let p = $head;
            let result = pg_combinators::Combinator::parse(&p, stream);
            let result = pg_parser_core::Optional::optional(result)?;
            if let Some(ok) = result {
                return Ok(ok)
            }

            $(
                let p = $tail;
                let result = pg_combinators::Combinator::parse(&p, stream);
                let result = pg_parser_core::Optional::optional(result)?;
                if let Some(ok) = result {
                    return Ok(ok)
                }
            )+

            Err(pg_parser_core::scan::Error::NoMatch(stream.current_location()))
        })
    };
}

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
        $crate::or(($head, $($tail),+))
    };
}
