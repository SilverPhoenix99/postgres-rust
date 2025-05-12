/// '+' | '-'
pub(super) fn sign() -> impl Combinator<Output = OperatorKind> {

    operator_if(|op| matches!(op, Minus | Plus))
}

use crate::parser::combinators::foundation::operator_if;
use crate::parser::combinators::foundation::Combinator;
use postgres_parser_lexer::OperatorKind;
use postgres_parser_lexer::OperatorKind::Minus;
use postgres_parser_lexer::OperatorKind::Plus;
