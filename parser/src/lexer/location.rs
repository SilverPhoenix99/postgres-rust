use std::ops::Range;

// A simple helper tuple type, to hold information related to some location.
pub type Located<T> = (T, Location);

// FIXME premature optimization:
//   This data is packed together considering generics in the future,
//   where it could be replaced with ()
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub range: Range<usize>,
    pub line: usize,
    pub col: usize,
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
}
