pub(super) fn opt_precision() -> impl Combinator<Output = Option<i32>> {
    i32_literal_paren()
        .optional()
}

use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::i32_literal_paren;
