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
        $crate::combinators::foundation::parser(move |stream| {
            #[allow(unused_imports)]
            use $crate::{
                combinators::foundation::Combinator,
                result::Required,
            };

            Ok((
                $head.parse(stream)?,
                $(
                    $tail.parse(stream).required()?,
                )+
            ))
        })
    };
}

pub(in crate::combinators) use seq;
