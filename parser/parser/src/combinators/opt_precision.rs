pub(super) fn opt_precision() -> impl Combinator<Output = Option<i32>> {
    i32_literal_paren()
        .optional()
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::i32_literal_paren;
