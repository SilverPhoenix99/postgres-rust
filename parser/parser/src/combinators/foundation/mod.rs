mod and;
mod combinator;
mod operator;

pub(crate) use self::combinator::Combinator;

pg_basics::reexport! { pub(in crate::combinators)
    between,
    bit_string,
    identifier,
    integer,
    keyword,
    located,
    many,
    map,
    number,
    optional,
    or,
    param,
    parser,
    skip,
    string,
    user_defined_operator,
}

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

#[allow(unused_imports)]
pub(crate) use alt;
pub(crate) use seq;
