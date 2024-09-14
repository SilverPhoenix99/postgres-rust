use crate::lexer::{Lexer, Location};
use std::ops::Range;

pub(super) struct TokenSpan<'lex, 'src> {
    lexer: &'lex Lexer<'src>,
    range: Range<usize>,
}

impl<'lex, 'src> TokenSpan<'lex, 'src>
where
    'src: 'lex
{
    pub fn new(lexer: &'lex Lexer<'src>, start_pos: usize) -> Self {

        let end_pos = lexer.buffer.current_index();

        debug_assert!(end_pos <= lexer.buffer.source().len());
        debug_assert!(start_pos <= end_pos);

        let range = start_pos..end_pos;
        Self { lexer, range }
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
