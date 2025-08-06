/**
Joins multiple parsers into a single parser.
* If all parsers return `Ok`, then a tuple with all results is returned.
* If any parser returns `Err`, then the parser returns that first `Err`.

Equivalent to `A & B ( & ... )*`.
*/
#[cfg(not(feature = "tuple_combinators"))]
#[macro_export]
macro_rules! seq {
    (
        $head:expr,
        $(
            $tail:expr
        ),+
        $(,)?
    ) => {
        $crate::parser(|stream| {

            let start_position = stream.current_location().range().start;

            Ok((
                {
                    let p = $head;
                    $crate::Combinator::parse(&p, stream)?
                },
                $({
                    let p = $tail;
                    let result = $crate::Combinator::parse(&p, stream);

                    match result {
                        Ok(ok) => ok,

                        Err(pg_parser_core::scan::Error::ScanErr(err))
                            => return Err(pg_parser_core::scan::Error::ScanErr(err)),

                        Err(pg_parser_core::scan::Error::Eof(loc) | pg_parser_core::scan::Error::NoMatch(loc)) => {
                            let current_position = stream.current_location().range().start;
                            return if start_position == current_position {
                                // No consumption yet, so this is considered the first production.
                                Err(pg_parser_core::scan::Error::NoMatch(loc))
                            } else {
                                // Otherwise, some consumed before, and this is not considered the first production.
                                // In this case, there was a partial match, and this is now considered a syntax error.
                                Err(pg_parser_core::syntax(loc))
                            }
                        }
                    }

                }),+
            ))
        })
    };
}

#[cfg(feature = "tuple_combinators")]
#[macro_export]
macro_rules! seq {
    (
        $head:expr,
        $(
            $tail:expr
        ),+
        $(,)?
    ) => {
        ($head, $($tail),+)
    };
}
