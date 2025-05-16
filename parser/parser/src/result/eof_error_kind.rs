#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EofErrorKind {
    NotEof(ParserError),
    Eof(Location),
}

impl_from!(LexerError for EofErrorKind::NotEof);
impl_from!(ParserError for EofErrorKind::NotEof);

use elog::lexer::LexerError;
use elog::parser::ParserError;
use postgres_basics::impl_from;
use postgres_basics::Location;
