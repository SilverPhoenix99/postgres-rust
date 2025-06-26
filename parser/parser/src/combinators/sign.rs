/// '+' | '-'
pub(super) fn sign(stream: &mut TokenStream) -> scan::Result<OperatorKind> {

    choice!(parsed stream =>
        Minus,
        Plus
    )
}

use crate::combinators::foundation::choice;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::OperatorKind;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::Plus;
