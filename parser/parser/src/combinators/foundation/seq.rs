macro_rules! seq {
    ($head:expr, $($tail:expr),+ $(,)?) => {
        $crate::combinators::foundation::parser(|stream| {
            #[allow(unused_imports)]
            use $crate::combinators::foundation::{Combinator, ClosureHelpers, CombinatorHelpers};
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
