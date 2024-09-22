use crate::Position;
use std::ops::Range;

/// A simple helper tuple type, to hold information related to some location.
pub type Located<T> = (T, Location);

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    range: Range<usize>,
    line: usize,
    col: usize,
}

impl Location {

    #[inline(always)]
    pub fn new(range: Range<usize>, line: usize, col: usize) -> Self {
        Self { range, line, col }
    }

    /// Slices the input source, according to the current range
    #[inline(always)]
    pub fn slice<'src>(&self, source: &'src [u8]) -> &'src [u8] {
        &source[self.range.clone()]
    }

    #[inline(always)]
    pub fn range(&self) -> &Range<usize> {
        &self.range
    }

    #[inline(always)]
    pub fn position(&self) -> Position {
        (self.line, self.col)
    }

    #[inline(always)]
    pub fn line(&self) -> usize {
        self.line
    }

    #[inline(always)]
    pub fn col(&self) -> usize {
        self.col
    }
}
