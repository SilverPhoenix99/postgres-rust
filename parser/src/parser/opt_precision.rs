use crate::parser::combinators::{Combinator, CombinatorHelpers};
use crate::parser::i32_literal_paren;

pub(in crate::parser) fn opt_precision() -> impl Combinator<Output = Option<i32>> {
    i32_literal_paren()
        .optional()
}