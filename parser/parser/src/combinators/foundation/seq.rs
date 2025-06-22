macro_rules! seq {
    ($head:expr, $($tail:expr),+ $(,)?) => {
        $crate::combinators::foundation::parser(|stream| {
            use $crate::combinators::foundation::Combinator;
            use $crate::result::Required;

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
