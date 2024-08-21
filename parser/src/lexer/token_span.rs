use crate::lexer::token_details::TokenDetails;
use crate::lexer::LexerError::InvalidTokenRange;
use crate::lexer::{DetailedError, Lexer};
use std::ops::Range;

pub struct TokenSpan<'lex, 'source> {
    lexer: &'lex Lexer<'source>,
    range: Range<usize>,
}

impl<'lex, 'source> TokenSpan<'lex, 'source> {

    pub fn new(lexer: &'lex Lexer<'source>, start_pos: usize) -> Result<Self, DetailedError> {

        let end_pos = lexer.buffer.current_index();
        let range = start_pos..end_pos;

        if start_pos > end_pos || end_pos > lexer.buffer.source().len() {
            let details = TokenDetails::new(range, lexer.buffer.location());
            return Err((InvalidTokenRange, details))
        }

        Ok(Self { lexer, range })
    }

    #[inline]
    pub fn details(&self) -> TokenDetails {
        TokenDetails::new(self.range.clone(), self.location())
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
