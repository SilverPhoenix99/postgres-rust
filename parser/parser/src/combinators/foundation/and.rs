/// Joins multiple parsers into a single parser.
/// * If all parsers return `Ok`, then a tuple with all results is returned.
/// * If any parser returns `Err`, then the parser returns that first `Err`.
///
/// Equivalent to `A & B ( & ... )*`.
macro_rules! tuple_and_combinator {
    (
        $(
            ($($t:ident : $f:tt),+ $(,)? )
        );+
        $(;)?
    ) => {

        $(
            impl<T0, $($t),+> Combinator for (T0, $($t),+)
            where
                T0: Combinator,
                $($t: Combinator),+
            {
                type Output = (T0::Output, $($t::Output),+);

                fn parse(&self, stream: &mut TokenStream) -> scan::Result<Self::Output> {
                    Ok((
                        self.0.parse(stream)?,
                        $(self.$f.parse(stream).required()?),+
                    ))
                }
            }
        )+
    };
}

tuple_and_combinator!(
    (T1:1);
    (T1:1, T2:2);
    (T1:1, T2:2, T3:3);
    (T1:1, T2:2, T3:3, T4:4);
    (T1:1, T2:2, T3:3, T4:4, T5:5);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6);
    (T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7);
);

use crate::combinators::foundation::combinator::Combinator;
use crate::result::Required;
use crate::scan;
use crate::stream::TokenStream;
