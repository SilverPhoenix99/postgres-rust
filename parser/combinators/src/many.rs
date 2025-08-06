/**
# Simple Form

Does `( P )+`.

## Return
If `Ok`, then the returned `Vec<_>` is **Never** empty.

Returns `Err(NoMatch)` or `Err(Eof)`, if empty.

# Separated Form

Does `P ( S P )*`, i.e., `P` separated by `S`.

`S` is discarded.

To do `P ( S Q )*`, where `P` and `Q` are different parsers returning the same type,
then use `many(pre = P, S Q)`, where `S` needs to be discarded in the `follow` parser.

## Return
If `Ok`, then the returned `Vec<_>` is **Never** empty.

Returns `Err(NoMatch)` or `Err(Eof)`, if empty.


# Prefixed Form

Does `P ( Q )*`, where both `P` and `Q` are different parsers, with the same output type.

If `P` and `Q` are the same parser, then use `many(P)` to prevent duplicate parsers.

## Return
If `Ok`, then the returned `Vec<_>` is **Never** empty.

Returns `Err(NoMatch)` or `Err(Eof)`, if empty.
*/
#[macro_export]
macro_rules! many {

    (pre = $prefix:expr, $follow:expr $(,)?) => {
        $crate::parser(|stream| {

            let p = $prefix;

            let first = $crate::Combinator::parse(&p, stream)?;
            let mut elements = vec![first];

            let p = $follow;

            while let Some(element) = {
                let result = $crate::Combinator::parse(&p, stream);
                pg_parser_core::Optional::optional(result)?
            } {
                elements.push(element)
            }

            Ok(elements)
        })
    };

    (sep = $separator:expr, $parser:expr $(,)?) => {
        $crate::parser(|stream| {

            let p = $parser;

            let first = $crate::Combinator::parse(&p, stream)?;
            let mut elements = vec![first];

            let separator = $separator;

            while {
                let sep = $crate::Combinator::parse(&separator, stream);
                let sep = pg_parser_core::Optional::optional(sep)?;
                sep.is_some()
            } {
                let element = $crate::Combinator::parse(&p, stream);
                let element = pg_parser_core::Required::required(element)?;
                elements.push(element);
            }

            Ok(elements)
        })
    };

    ($parser:expr) => {
        $crate::parser(|stream| {

            let p = $parser;

            let first = $crate::Combinator::parse(&p, stream)?;
            let mut elements = vec![first];

            while let Some(element) = {
                let result = $crate::Combinator::parse(&p, stream);
                pg_parser_core::Optional::optional(result)?
            } {
                elements.push(element)
            }

            Ok(elements)
        })
    };
}
