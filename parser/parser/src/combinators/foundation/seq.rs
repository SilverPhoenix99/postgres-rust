macro_rules! seq {
    ($head:expr, $($tail:expr),+ $(,)?) => {
        (|| -> $crate::scan::Result<_> {
            use $crate::result::Required;

            Ok((
                $head?,
                $(
                    $tail.required()?,
                )+
            ))
        })()
    };
}

pub(in crate::combinators) use seq;
