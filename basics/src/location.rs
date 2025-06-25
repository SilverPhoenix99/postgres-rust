/// A simple helper tuple type, to hold information related to some location.
pub type Located<T> = (T, Location);

// The limit for stmt strings is 1gb
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Location {
    range: Range<u32>, // in bytes
    line: u32,
    col: u32,
}

impl Location {

    pub fn new(range: Range<u32>, line: u32, col: u32) -> Self {
        Self { range, line, col }
    }

    /// Slices the input source, according to the current range
    pub fn slice<'src>(&self, source: &'src str) -> &'src str {
        let range = self.range.start as usize..self.range.end as usize;
        &source[range]
    }

    pub fn range(&self) -> &Range<u32> {
        &self.range
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn col(&self) -> u32 {
        self.col
    }

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
use core::ops::Range;
