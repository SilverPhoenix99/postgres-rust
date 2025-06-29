pub(in crate::combinators) fn located<P>(combinator: P) -> impl Combinator<Output = Located<P::Output>>
where
    P: Combinator
{
    parser(move |stream| {
        let loc = stream.current_location();
        let result = combinator.parse(stream)?;
        Ok((result, loc))
    })
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use pg_basics::Located;
