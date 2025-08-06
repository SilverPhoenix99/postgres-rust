#![cfg(feature = "tuple_combinators")]

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

                    let start_position = stream.current_location().range().start;

                    Ok((

                        self.0.parse(stream)?,

                        $({
                            match self.$f.parse(stream) {
                                Ok(ok) => ok,

                                Err(ScanErr(err)) => return Err(ScanErr(err)),

                                Err(Eof(loc) | NoMatch(loc)) => {
                                    let current_position = stream.current_location().range().start;
                                    return if start_position == current_position {
                                        // No consumption yet, so this is considered the first production.
                                        Err(NoMatch(loc))
                                    } else {
                                        // Otherwise, some consumed before, and this is not considered the first production.
                                        // In this case, there was a partial match, and this is now considered a syntax error.
                                        Err(syntax(loc))
                                    }
                                }
                            }
                        }),+
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

use crate::Combinator;
use pg_parser_core::scan;
use pg_parser_core::scan::Error::Eof;
use pg_parser_core::scan::Error::NoMatch;
use pg_parser_core::scan::Error::ScanErr;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::syntax;
