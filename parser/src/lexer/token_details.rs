use std::ops::Range;

// TODO premature opt:
//   This data is packed together considering generics in the future,
//   where it could be replaced with ()
#[derive(Debug, Clone, PartialEq)]
pub struct TokenDetails {
    pub range: Range<usize>,
    pub location: (usize, usize)
}

impl TokenDetails {

    pub fn new(range: Range<usize>, location: (usize, usize)) -> Self {
        Self { range, location }
    }
}
