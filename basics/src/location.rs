/// A simple helper tuple type, to hold information related to some location.
pub type Located<T> = (T, Location);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Location {
    range: Range<usize>, // in bytes
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
    pub fn slice<'src>(&self, source: &'src str) -> &'src str {
        &source[self.range.clone()]
    }

    #[inline(always)]
    pub fn range(&self) -> &Range<usize> {
        &self.range
    }

    #[inline(always)]
    pub fn line(&self) -> usize {
        self.line
    }

    #[inline(always)]
    pub fn col(&self) -> usize {
        self.col
    }

    #[inline(always)]
    pub fn position(&self) -> Position {
        (self.line, self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let loc = Location::new(0..0, 3, 5);
        assert_eq!((3, 5), loc.position())
    }
}

use crate::Position;
use std::ops::Range;
