/// '+' | '-'
pub(super) fn sign() -> impl Combinator<Output = OperatorKind> {

    operator_if(|op| matches!(op, Minus | Plus))
}

use crate::combinators::foundation::operator_if;
use crate::combinators::foundation::Combinator;
use pg_lexer::OperatorKind;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::Plus;
