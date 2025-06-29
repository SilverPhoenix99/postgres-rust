/// '+' | '-'
pub(super) fn sign(stream: &mut TokenStream) -> scan::Result<OperatorKind> {

    or((Minus, Plus))
        .parse(stream)
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::OperatorKind;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::Plus;
