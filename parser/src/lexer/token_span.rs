use crate::lexer::LexerErrorCode::InvalidTokenRange;
use crate::lexer::{Lexer, LexerError};
use std::ops::Range;

pub struct TokenSpan<'lex, 'source> {
    lexer: &'lex Lexer<'source>,
    range: Range<usize>,
}

impl<'lex, 'source> TokenSpan<'lex, 'source> {

    pub fn new(lexer: &'lex Lexer<'source>, start_pos: usize) -> Result<Self, LexerError> {

        let end_pos = lexer.buffer.current_index();
        let range = start_pos..end_pos;

        if start_pos > end_pos || end_pos > lexer.buffer.source().len() {
            return Err(LexerError::new(InvalidTokenRange, (range, lexer.buffer.location())))
        }

        Ok(Self { lexer, range })
    }

    #[inline]
    pub fn details(&self) -> (Range<usize>, (usize, usize)) {
        (self.range.clone(), self.location())
    }

    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    #[inline]
    pub fn location(&self) -> (usize, usize) {
        self.lexer.buffer.location_at(self.range.start)
    }

    #[inline]
    pub fn slice(&self) -> &'source [u8] {
        let source = self.lexer.buffer.source();
        &source[self.range()]
    }
}
