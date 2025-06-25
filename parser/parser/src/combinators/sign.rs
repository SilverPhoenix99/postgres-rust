/// '+' | '-'
pub(super) fn sign(stream: &mut TokenStream) -> Result<OperatorKind> {

    operator_if(|op| matches!(op, Minus | Plus))
        .parse(stream)
}

use crate::combinators::foundation::operator_if;
use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_lexer::OperatorKind;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::Plus;
