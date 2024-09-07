use std::ops::Range;

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

    #[inline(always)]
    pub fn of<T>(self, value: T) -> Locatable<T> {
        Locatable::new(value, self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Locatable<T>(pub T, pub Location);

impl<T> Locatable<T> {

    #[inline(always)]
    pub fn new(value: T, loc: Location) -> Self {
        Self(value, loc)
    }

    #[inline(always)]
    pub fn value(&self) -> &T {
        &self.0
    }

    #[inline(always)]
    pub fn loc(&self) -> &Location {
        &self.1
    }
}
