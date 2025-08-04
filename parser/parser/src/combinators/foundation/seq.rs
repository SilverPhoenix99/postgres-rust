/// Joins multiple parsers into a single parser.
/// * If all parsers return `Ok`, then a tuple with all results is returned.
/// * If any parser returns `Err`, then the parser returns that first `Err`.
///
/// Equivalent to `A & B ( & ... )*`.
macro_rules! seq {
    (
        $head:expr,
        $(
            $tail:expr
        ),+
        $(,)?
    ) => {
        $crate::combinators::foundation::parser(|stream| {
            Ok((
                {
                    let p = $head;
                    $crate::combinators::foundation::Combinator::parse(&p, stream)?
                },
                $({
                    let p = $tail;
                    let result = $crate::combinators::foundation::Combinator::parse(&p, stream);
                    $crate::result::Required::required(result)?
                }),+
            ))
        })
    };
}

pub(in crate::combinators) use seq;
