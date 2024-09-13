use crate::lexer::LexerError::InvalidTokenRange;
use crate::lexer::{Lexer, Locatable, LocatableError, Location};
use std::ops::Range;

pub struct TokenSpan<'lex, 'src> {
    lexer: &'lex Lexer<'src>,
    range: Range<usize>,
}

impl<'lex, 'src> TokenSpan<'lex, 'src>
where
    'src: 'lex
{
    pub fn new(lexer: &'lex Lexer<'src>, start_pos: usize) -> Result<Self, LocatableError> {

        let end_pos = lexer.buffer.current_index();
        let range = start_pos..end_pos;

        if start_pos > end_pos || end_pos > lexer.buffer.source().len() {
            let (line, col) = lexer.buffer.location();
            let details = Location::new(range, line, col);
            return Err(Locatable::new(InvalidTokenRange, details))
        }

        Ok(Self { lexer, range })
    }

    #[inline]
    pub fn location(&self) -> Location {
        let (line, col) = self.lexer.buffer.location_at(self.range.start);
        Location::new(self.range.clone(), line, col)
    }

    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    #[inline]
    pub fn slice(&self) -> &'src [u8] {
        let source = self.lexer.buffer.source();
        &source[self.range()]
    }
}
