/// '+' | '-'
pub(super) fn sign(stream: &mut TokenStream) -> scan::Result<OperatorKind> {

    alt!(Minus, Plus)
        .parse(stream)
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::Combinator;
use pg_lexer::OperatorKind;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::Plus;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
