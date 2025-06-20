/// Aliases:
/// * `ColId`
/// * `name`
pub(super) fn col_id() -> impl Combinator<Output = Str> {
    parser(v2::col_id)
}

use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::v2;
use pg_basics::Str;
