pub(in crate::parser) fn opt_precision() -> impl Combinator<Output = Option<i32>> {
    i32_literal_paren()
        .optional()
}

use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::i32_literal_paren;
