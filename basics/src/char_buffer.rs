/// A tuple of `(line, column)`.
pub type Position = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnicodeChar {
    SurrogateFirst(u16),
    SurrogateSecond(u16),
    Utf8(char),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnicodeCharError {

    /// When the number of bytes doesn't match the expected number.
    LenTooShort,

    /// When it's an invalid UTF-32 char (and invalid UTF-16 surrogate).
    InvalidUnicodeValue,
}

#[derive(Debug, Clone)]
struct LineBuffer {
    lines: Vec<usize> // where each line begins, in bytes (i.e., col 1)
}

#[derive(Debug, Clone)]
pub struct CharBuffer<'src> {
    source: &'src str,
    current_index: usize, // in bytes
    lines: LineBuffer
}

impl<'src> CharBuffer<'src> {

    #[inline]
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            current_index: 0,
            lines: LineBuffer::new()
        }
    }

    #[inline(always)]
    pub fn source(&self) -> &'src str {
        self.source
    }

    #[inline(always)]
    pub fn remainder(&self) -> &'src str {
        &self.source[self.current_index..]
    }

    #[inline(always)]
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    #[inline(always)]
    pub fn current_position(&self) -> Position {
        self.position_at(self.current_index)
    }

    #[inline(always)]
    pub fn position_at(&self, index: usize) -> Position {
        self.lines.position(index)
    }

    /// Location's range will always be zero-length.
    #[inline(always)]
    pub fn current_location(&self) -> Location {
        self.location_starting_at(self.current_index)
    }

    /// Panics when `start_index > self.current_index()`.
    #[inline]
    pub fn location_starting_at(&self, start_index: usize) -> Location {
        assert!(start_index <= self.current_index, "start_index shouldn't be past current_index");
        let (line, col) = self.lines.position(start_index);
        Location::new(start_index..self.current_index, line, col)
    }

    /// Panics if `start_index > self.current_index()`, or `start_index` is not at a start of a char.
    #[inline]
    pub fn slice(&self, start_index: usize) -> &'src str {
        assert!(start_index <= self.current_index, "start_index shouldn't be past current_index");
        assert!(self.source.is_char_boundary(start_index), "start_index must be at the 1st byte of a UTF-8 char");
        &self.source[start_index..self.current_index]
    }

    pub fn peek(&self) -> Option<char> {
        if self.eof() {
            None
        }
        else {
            self.remainder().chars().next()
        }
    }

    #[inline(always)]
    pub fn eof(&self) -> bool {
        self.current_index == self.source.len()
    }

    /// Consumes a char if it matches `expected`.
    /// Returns `true`, if the char matched.
    #[inline(always)]
    pub fn consume_char(&mut self, expected: char) -> bool {
        self.consume_if(|c| c == expected).is_some()
    }

    /// Consumes a char if one is available (non-eof) and `pred` returns `true`.
    #[inline]
    pub fn consume_if(&mut self, pred: impl FnOnce(char) -> bool) -> Option<char> {
        if self.peek().is_some_and(pred) {
            return self.consume_one();
        }
        None
    }

    /// Consumes chars while they're available and `pred` returns `true`.
    /// Returns the number of **bytes** (not chars) successfully consumed.
    pub fn consume_while(&mut self, pred: impl Fn(char) -> bool) -> usize {

        let start_index = self.current_index;
        let chars = self.remainder()
            .chars()
            .take_while(|c| pred(*c));
        for c in chars {
            self.buffer_new_line();
            self.current_index += c.len_utf8();
        }

        self.current_index - start_index
    }

    /// Unconditionally consumes a char, if available.
    pub fn consume_one(&mut self) -> Option<char> {

        self.buffer_new_line();
        let c = self.peek()?;
        self.current_index += c.len_utf8();

        Some(c)
    }

    /// Consumes as many chars as there are available, up to `num_chars`.
    pub fn consume_many(&mut self, num_chars: usize) -> &'src str {

        let start_index = self.current_index;
        let remainder = self.remainder();
        let chars = remainder.chars()
            .take(num_chars);
        for c in chars {
            self.buffer_new_line();
            self.current_index += c.len_utf8();
        }

        let len = self.current_index - start_index;
        &remainder[..len]
    }

    fn buffer_new_line(&mut self) {

        let mut chars = self.remainder().chars();
        let Some(c) = chars.next() else { return };

        if c == '\n' {
            // Unix style LF
            self.lines.push(self.current_index + 1); // '\n' => len 1
            return
        }

        if c != '\r' {
            return
        }

        let Some(c) = chars.next() else { return };

        if c != '\n' {
            // Old Mac style CR
            // Push only if not followed by a \n.
            self.lines.push(self.current_index + 1 + c.len_utf8())  // '\r' => len 1
        }

        // Windows style CRLF
        // This will be pushed on the next advance_char, so do nothing here.
    }

    #[inline(always)]
    pub fn push_back(&mut self) {

        if self.current_index == 0 {
            return
        }

        let lead = &self.source[0..self.current_index];
        let Some(c) = lead.chars().next_back() else { return };

        self.current_index -= c.len_utf8();
    }

    #[inline]
    /// Use sparingly!
    ///
    /// Panics if `index` is not at a char boundary (i.e., 1st byte of a char)
    pub fn seek(&mut self, index: usize) {
        let index = min(index, self.source.len());
        self.current_index = index;
    }

    #[inline]
    pub fn consume_string(&mut self, expected: &str) -> bool {

        if self.remainder().starts_with(expected) {
            self.current_index += expected.len();
            return true
        }
        false
    }

    /// Consume a hexadecimal representation of a Unicode char,
    /// represented in either in UTF-16 or UTF-32.
    ///
    /// E.g.: `"0061"` outputs `Utf8('a')`
    pub fn consume_unicode_char(&mut self, unicode_len: usize) -> Result<UnicodeChar, UnicodeCharError> {

        debug_assert!(unicode_len <= 8, "unicode encoded chars should be <= 8 chars");

        let slice = self.remainder();
        if slice.len() < unicode_len {
            return Err(LenTooShort)
        }

        let slice = &slice[..unicode_len];

        let Ok(c) = u32::from_str_radix(slice, 16) else { return Err(LenTooShort) };

        if c == 0 {
            // PG doesn't like this char, because it breaks c strings
            return Err(InvalidUnicodeValue)
        }

        let result = decode_unicode(c)
            .ok_or(InvalidUnicodeValue)?;

        self.consume_many(unicode_len);
        Ok(result)
    }
}

#[inline(always)]
fn decode_unicode(c: u32) -> Option<UnicodeChar> {

    if c <= 0xffff {
        if is_utf16_surrogate_first(c as u16) {
            return Some(SurrogateFirst(c as u16))
        }
        if is_utf16_surrogate_second(c as u16) {
            return Some(SurrogateSecond(c as u16))
        }
    }

    char::from_u32(c).map(Utf8)
}

impl Default for LineBuffer {

    #[inline]
    fn default() -> Self {
        let mut lines = Vec::with_capacity(8);
        lines.push(0);
        Self { lines }
    }
}

/// Saves the indexes of lines (LF/CR/CRLF),
/// and calculates positions (line + column) from indexes.
impl LineBuffer {

    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Saves the index where a line begins.
    /// This means that the index matches where column 1 is on that line.
    pub fn push(&mut self, line_start_index: usize) {

        if
            self.lines.is_empty()
            || line_start_index > *self.lines.last().unwrap()
        {
            // fast path
            self.lines.push(line_start_index);
            return
        }

        if let Err(index) = self.lines.binary_search(&line_start_index) {
            self.lines.insert(index, line_start_index)
        }
    }

    /// Calculates the line and column from a given index.
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
mod tests {
    use super::*;

    #[test]
    fn test_remainder() {
        let buffer = CharBuffer {
            current_index: 1,
            source: "abc",
            lines: LineBuffer::default()
        };

        assert_eq!("bc", buffer.remainder());
    }

    #[test]
    fn test_current_position() {
        let buffer = CharBuffer {
            current_index: 1,
            source: "abc",
            lines: LineBuffer::default()
        };

        assert_eq!((1, 2), buffer.current_position());
    }

    #[test]
    fn test_position_at() {
        let buffer = CharBuffer::new("abc");

        assert_eq!((1, 3), buffer.position_at(2));
    }

    #[test]
    fn test_current_location() {
        let buffer = CharBuffer {
            current_index: 1,
            source: "abc",
            lines: LineBuffer::default()
        };

        assert_eq!(Location::new(1..1, 1, 2), buffer.current_location());
    }

    #[test]
    fn test_location_starting_at() {
        let buffer = CharBuffer {
            current_index: 1,
            source: "abc",
            lines: LineBuffer::default()
        };

        assert_eq!(Location::new(0..1, 1, 1), buffer.location_starting_at(0));
    }

    #[test]
    fn test_slice() {
        let buffer = CharBuffer {
            current_index: 2,
            source: "abc",
            lines: LineBuffer::default()
        };

        assert_eq!("ab", buffer.slice(0));
    }

    #[test]
    fn test_peek() {
        let buffer = CharBuffer {
            current_index: 1,
            source: "abc",
            lines: LineBuffer::default()
        };

        assert_eq!(Some('b'), buffer.peek());
    }

    #[test]
    fn test_eof() {
        let buffer = CharBuffer {
            current_index: 1,
            source: "abc",
            lines: LineBuffer::default()
        };

        assert!(!buffer.eof());

        let buffer = CharBuffer {
            current_index: 3,
            source: "abc",
            lines: LineBuffer::default()
        };

        assert!(buffer.eof());
    }

    #[test]
    fn test_consume_char() {
        let mut buffer = CharBuffer::new("abc");

        assert!(!buffer.consume_char('b'));
        assert!(buffer.consume_char('a'));
    }

    #[test]
    fn test_consume_if() {
        let mut buffer = CharBuffer::new("abc");

        assert_eq!(None, buffer.consume_if(|c| c == 'b'));
        assert_eq!(Some('a'), buffer.consume_if(|c| c == 'a'));
    }

    #[test]
    fn test_consume_while() {
        let mut buffer = CharBuffer::new("aaabc");

        assert_eq!(0, buffer.consume_while(|c| c == 'b'));
        assert_eq!(3, buffer.consume_while(|c| c == 'a'));
    }

    #[test]
    fn test_consume_one() {
        let mut buffer = CharBuffer::new("a");

        assert_eq!(Some('a'), buffer.consume_one());
        assert_eq!(None, buffer.consume_one());
    }

    #[test]
    fn test_consume_many() {
        let mut buffer = CharBuffer::new("abc");

        assert_eq!("ab", buffer.consume_many(2));
        assert_eq!("c", buffer.consume_many(2));
        assert_eq!("", buffer.consume_many(2));
    }

    #[test]
    fn test_push_back() {
        let mut buffer = CharBuffer {
            current_index: 1,
            source: "abc",
            lines: LineBuffer::default()
        };

        assert_eq!(Some('b'), buffer.peek());
        buffer.push_back();
        assert_eq!(Some('a'), buffer.peek());
    }

    #[test]
    fn test_seek() {
        let mut buffer = CharBuffer {
            current_index: 2,
            source: "abc",
            lines: LineBuffer::default()
        };

        assert_eq!(Some('c'), buffer.peek());
        buffer.seek(0);
        assert_eq!(Some('a'), buffer.peek());
    }

    #[test]
    fn test_consume_string() {
        let mut buffer = CharBuffer::new("abc");

        assert!(!buffer.consume_string("bc"));
        assert!(buffer.consume_string("ab"));
    }

    #[test]
    fn test_consume_unicode_char() {
        let mut buffer = CharBuffer::new("0000640061");

        assert_eq!(Ok(Utf8('d')), buffer.consume_unicode_char(6));
        assert_eq!(Ok(Utf8('a')), buffer.consume_unicode_char(4));
    }

    #[test]
    fn test_line_buffer_position() {
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

use crate::{
    wchar::is_utf16_surrogate_first,
    wchar::is_utf16_surrogate_second,
    Location
};
use std::cmp::min;
use UnicodeChar::*;
use UnicodeCharError::*;
