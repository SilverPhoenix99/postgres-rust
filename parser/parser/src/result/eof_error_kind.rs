#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EofErrorKind {
    NotEof(ParserError),
    Eof(Location),
}

impl_from!(LexerError for EofErrorKind::NotEof);
impl_from!(ParserError for EofErrorKind::NotEof);

use pg_basics::impl_from;
use pg_basics::Location;
use pg_elog::lexer::LexerError;
use pg_elog::parser::ParserError;
