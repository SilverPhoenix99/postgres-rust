use std::cmp::min;

#[derive(Debug, Clone)]
struct LineBuffer {
    lines: Vec<usize> // where each line begins (i.e., col 1)
}

#[derive(Debug)]
pub struct CharBuffer<'src> {
    source: &'src [u8],
    current_index: usize,
    lines: LineBuffer
}

impl<'src> CharBuffer<'src> {

    #[inline]
    pub fn new(source: &'src [u8]) -> Self {
        Self {
            source,
            current_index: 0,
            lines: LineBuffer::new()
        }
    }

    #[inline(always)]
    pub fn source(&self) -> &'src [u8] {
        self.source
    }

    #[inline(always)]
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    #[inline(always)]
    pub fn location(&self) -> (usize, usize) {
        self.location_at(self.current_index)
    }

    #[inline(always)]
    pub fn location_at(&self, position: usize) -> (usize, usize) {
        self.lines.location(position)
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.eof() { None } else { Some(self.peek_unsafe()) }
    }

    #[inline(always)]
    pub fn eof(&self) -> bool {
        self.current_index == self.source.len()
    }

    #[inline(always)]
    pub(crate) fn peek_unsafe(&self) -> u8 {
        self.source[self.current_index]
    }

    /// Consumes a char if it matches `expected`.
    /// Returns `true`, if the char matched.
    #[inline(always)]
    pub fn consume_char(&mut self, expected: u8) -> bool {
        self.consume_if(|c| c == expected)
    }

    /// Consumes a char if one is available (non-eof) and `pred` returns `true`.
    #[inline]
    pub fn consume_if(&mut self, pred: impl FnOnce(u8) -> bool) -> bool {
        if self.peek().is_some_and(pred) {
            self.consume_one();
            return true
        }
        false
    }

    /// Consumes chars while they're available and `pred` returns `true`.
    /// Returns the number of chars successfully consumed.
    #[inline]
    pub fn consume_while(&mut self, pred: impl Fn(u8) -> bool) -> usize
    {
        let mut consumed = 0;
        while self.consume_if(&pred) {
            consumed += 1;
        }
        consumed
    }

    /// Unconditionally consumes a char, if available.
    pub fn consume_one(&mut self) -> Option<u8> {

        let c = self.peek()?;
        self.buffer_new_line(self.current_index);
        self.current_index += 1;

        Some(c)
    }

    /// Consumes as many chars as there are available, up to `num_chars`.
    pub fn consume_many(&mut self, num_chars: usize) -> &'src [u8] {

        let start_pos = self.current_index;
        let end_pos = min(start_pos + num_chars, self.source.len());
        self.current_index = end_pos;

        &self.source[start_pos..end_pos]
    }

    fn buffer_new_line(&mut self, pos: usize) {

        let c = self.source[pos];
        if c == b'\n' {
            // Unix style LF
            self.lines.push(pos + 1);
            return
        }

        if c != b'\r' {
            return
        }

        let pos = pos + 1;
        if pos >= self.source.len() || self.source[pos] != b'\n' {
            // Old Mac style CR
            // Push only if not followed by a \n.
            self.lines.push(pos + 1)
        }

        // Windows style CRLF
        // This will be pushed on the next advance_char, so do nothing here.
    }

    #[inline(always)]
    pub fn push_back(&mut self) {
        self.current_index -= 1;
    }

    #[inline]
    /// Use sparingly!
    pub fn seek(&mut self, index: usize) {
        let pos = min(index, self.source.len());
        self.current_index = pos;
    }

    pub fn lookahead(&self, expected: &[u8]) -> bool {
        let len = self.source.len() - self.current_index;
        if len < expected.len() {
            return false
        }

        let end_pos = self.current_index + expected.len();
        let span = self.current_index..end_pos;
        let actual = &self.source[span];

        actual == expected
    }

    #[inline]
    pub fn consume_string(&mut self, expected: &[u8]) -> bool {

        if self.lookahead(expected) {
            self.consume_many(expected.len());
            return true
        }
        false
    }
}

/// Saves the position of lines (LF/CR/CRLF),
/// and calculates location (line + column) from indexes.
impl LineBuffer {

    #[inline]
    pub fn new() -> Self {
        let mut lines = Vec::with_capacity(8);
        lines.push(0);
        Self { lines }
    }

    pub fn push(&mut self, line: usize) {

        if
            self.lines.is_empty()
            || line > *self.lines.last().unwrap()
        {
            // fast path
            self.lines.push(line);
            return
        }

        if let Err(index) = self.lines.binary_search(&line) {
            self.lines.insert(index, line)
        }
    }

    /// Calculates the line and column from a given position.
    pub fn location(&self, position: usize) -> (usize, usize) {

        match self.lines.binary_search(&position) {
            Ok(index) => (index + 1, 1),
            Err(line) => {
                let line_start = self.lines[line - 1];
                (line, position - line_start + 1)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_location() {
        let buf = LineBuffer { lines: vec![0, 10, 23] };

        assert_eq!((1, 1), buf.location(0));
        assert_eq!((1, 6), buf.location(5));
        assert_eq!((2, 1), buf.location(10));
        assert_eq!((2, 3), buf.location(12));
        assert_eq!((2, 8), buf.location(17));
        assert_eq!((3, 1), buf.location(23));
        assert_eq!((3, 5), buf.location(27));
    }
}
