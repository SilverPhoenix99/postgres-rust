use crate::{wchar, Location};
use std::cmp::min;
use UnicodeChar::*;
use UnicodeCharError::*;

pub type Position = (usize, usize);

pub enum UnicodeChar {
    SurrogateFirst(u16),
    SurrogateSecond(u16),
    Utf8(char),
}

pub enum UnicodeCharError {

    /// When the number of bytes doesn't match the expected number.
    LenTooShort(/* actual_len: */usize),

    /// When it's an invalid UTF-32 char (and invalid UTF-16 surrogate).
    InvalidUnicodeValue,
}

#[derive(Debug, Clone)]
#[repr(transparent)]
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
    pub fn remainder(&self) -> &'src [u8] {
        &self.source[self.current_index..]
    }

    #[inline(always)]
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    #[inline(always)]
    pub fn position(&self) -> Position {
        self.position_at(self.current_index)
    }

    #[inline(always)]
    pub fn position_at(&self, index: usize) -> Position {
        self.lines.position(index)
    }

    /// Location's range will always be zero-length.
    #[inline(always)]
    pub fn location(&self) -> Location {
        self.location_starting_at(self.current_index)
    }

    /// Panics when `start_index > self.current_index()`.
    #[inline]
    pub fn location_starting_at(&self, start_index: usize) -> Location {
        assert!(start_index <= self.current_index);
        let (line, col) = self.lines.position(start_index);
        Location::new(start_index..self.current_index, line, col)
    }

    pub fn slice(&self, start_index: usize) -> &'src [u8] {
        assert!(start_index <= self.current_index);
        &self.source[start_index..self.current_index]
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.eof() {
            None
        }
        else {
            Some(self.source[self.current_index])
        }
    }

    #[inline(always)]
    pub fn eof(&self) -> bool {
        self.current_index == self.source.len()
    }

    /// Consumes a char if it matches `expected`.
    /// Returns `true`, if the char matched.
    #[inline(always)]
    pub fn consume_char(&mut self, expected: u8) -> bool {
        self.consume_if(|c| c == expected).is_some()
    }

    /// Consumes a char if one is available (non-eof) and `pred` returns `true`.
    #[inline]
    pub fn consume_if(&mut self, pred: impl FnOnce(u8) -> bool) -> Option<u8> {
        if self.peek().is_some_and(pred) {
            return self.consume_one();
        }
        None
    }

    /// Consumes chars while they're available and `pred` returns `true`.
    /// Returns the number of chars successfully consumed.
    #[inline]
    pub fn consume_while(&mut self, pred: impl Fn(u8) -> bool) -> usize
    {
        let mut consumed = 0;
        while self.consume_if(&pred).is_some() {
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

        let start_index = self.current_index;
        let end_index = min(start_index + num_chars, self.source.len());
        self.current_index = end_index;

        &self.source[start_index..end_index]
    }

    fn buffer_new_line(&mut self, index: usize) {

        let c = self.source[index];
        if c == b'\n' {
            // Unix style LF
            self.lines.push(index + 1);
            return
        }

        if c != b'\r' {
            return
        }

        let index = index + 1;
        if index >= self.source.len() || self.source[index] != b'\n' {
            // Old Mac style CR
            // Push only if not followed by a \n.
            self.lines.push(index + 1)
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
        let index = min(index, self.source.len());
        self.current_index = index;
    }

    #[inline]
    pub fn consume_string(&mut self, expected: &[u8]) -> bool {

        if self.remainder().starts_with(expected) {
            self.consume_many(expected.len());
            return true
        }
        false
    }

    /// Consume a hexadecimal representation of a Unicode char,
    /// represented in either in UTF-16 or UTF-32.
    ///
    /// E.g.: `"0061"` outputs `Utf8('a')`
    pub fn consume_unicode_char(&mut self, unicode_len: usize) -> Result<UnicodeChar, UnicodeCharError> {

        debug_assert!(unicode_len <= 8, "unicode encoded chars cannot be longer than 8 chars");

        let slice = self.remainder();
        if slice.len() < unicode_len {
            return Err(LenTooShort(slice.len()))
        }

        let c = slice[..unicode_len]
            .iter()
            .enumerate()
            .map(|(n, d)| {
                char::from(*d)
                    .to_digit(16)
                    .ok_or(LenTooShort(n))
            })
            .try_fold(0, |acc, d|
                Ok((acc << 4) + d?)
            )?;

        let result = {
            if wchar::is_utf16_surrogate_first(c as u16) {
                Ok(SurrogateFirst(c as u16))
            } else if wchar::is_utf16_surrogate_second(c as u16) {
                Ok(SurrogateSecond(c as u16))
            }
            else {
                char::from_u32(c)
                    .map(Utf8)
                    .ok_or(InvalidUnicodeValue)
            }
        };

        if result.is_ok() {
            self.consume_many(unicode_len);
        }

        result
    }
}

impl Default for LineBuffer {

    #[inline]
    fn default() -> Self {
        let mut lines = Vec::with_capacity(8);
        lines.push(0);
        Self { lines }
    }
}

/// Saves the position of lines (LF/CR/CRLF),
/// and calculates location (line + column) from indexes.
impl LineBuffer {

    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
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
    pub fn position(&self, index: usize) -> Position {

        match self.lines.binary_search(&index) {
            Ok(index) => (index + 1, 1),
            Err(line) => {
                let line_start = self.lines[line - 1];
                (line, index - line_start + 1)
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

        assert_eq!((1, 1), buf.position(0));
        assert_eq!((1, 6), buf.position(5));
        assert_eq!((2, 1), buf.position(10));
        assert_eq!((2, 3), buf.position(12));
        assert_eq!((2, 8), buf.position(17));
        assert_eq!((3, 1), buf.position(23));
        assert_eq!((3, 5), buf.position(27));
    }
}
