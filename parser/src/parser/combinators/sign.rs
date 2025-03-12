/// '+' | '-'
pub(super) fn sign() -> impl Combinator<Output = OperatorKind> {
    use crate::lexer::OperatorKind::{Minus, Plus};
    use crate::parser::combinators::foundation::operator_if;
    operator_if(|op| matches!(op, Minus | Plus))
}

use crate::lexer::OperatorKind;
use crate::parser::combinators::foundation::Combinator;
