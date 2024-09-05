mod ascii_flags;
mod lexer_buffer;
mod lexer_error;
mod locatable;
mod token_kind;
mod token_span;

pub use crate::lexer::ascii_flags::*;
pub use crate::lexer::lexer_buffer::LexerBuffer;
pub use crate::lexer::lexer_error::LexerError;
use crate::lexer::lexer_error::LexerError::*;
pub use crate::lexer::locatable::{Locatable, Location};
use crate::lexer::token_kind::IdentifierKind::*;
use crate::lexer::token_kind::StringKind::*;
use crate::lexer::token_kind::TokenKind::*;
pub use crate::lexer::token_kind::{IdentifierKind, StringKind, TokenKind};
pub use crate::lexer::token_span::TokenSpan;
use postgres_basics::NAMEDATALEN;
use std::iter::FusedIterator;

type LexResult = Result<TokenKind, LexerError>;

pub(crate) type LocatableToken = Locatable<TokenKind>;
type LocatableError = Locatable<LexerError>;
pub(crate) type LexerResult = Result<LocatableToken, LocatableError>;

#[derive(Debug)]
pub struct Lexer<'source> {
    buffer: LexerBuffer<'source>,
    standard_conforming_strings: bool
}

impl<'source> Iterator for Lexer<'source> {
    type Item = LexerResult;

    /// The token is always a full match,
    /// never a substring that's more interesting that the whole match.
    fn next(&mut self) -> Option<Self::Item> {

        let concatenable_whitespace = match self.skip_trivia() {
            Ok(concatenable_whitespace) => concatenable_whitespace,
            Err(err) => return Some(Err(err))
        };

        if self.buffer.eof() {
            return None
        }

        let start_pos = self.buffer.current_index();

        let token = self.lex_token(concatenable_whitespace);

        let location = TokenSpan::new(self, start_pos)
            .unwrap()
            .location();

        match token {
            Ok(kind) => Some(Ok(location.of(kind))),
            Err(err_code) => Some(Err(location.of(err_code))),
        }
    }
}

impl<'source> FusedIterator for Lexer<'source> {}

impl<'source> Lexer<'source> {

    #[inline]
    pub fn new(source: &'source [u8], standard_conforming_strings: bool) -> Self {
        Self {
            buffer: LexerBuffer::new(source),
            standard_conforming_strings
        }
    }

    fn lex_token(&mut self, concatenable_string: bool) -> LexResult {

        // eof has already been filtered
        match self.buffer.advance_char().unwrap() {
            b'(' => Ok(OpenParenthesis),
            b')' => Ok(CloseParenthesis),
            b',' => Ok(Comma),
            b';' => Ok(Semicolon),
            b'[' => Ok(OpenBracket),
            b']' => Ok(CloseBracket),
            b'.' => {
                if self.buffer.consume(b'.') {
                    Ok(DotDot)
                }
                else if self.buffer.peek().is_some_and(is_decimal_digit) {
                    self.lex_dec_float()
                }
                else {
                    Ok(Dot)
                }
            }
            b':' => {
                if self.buffer.consume(b':') {
                    Ok(Typecast)
                }
                else if self.buffer.consume(b'=') {
                    Ok(ColonEquals)
                }
                else {
                    Ok(Colon)
                }
            }
            b'$' => match self.buffer.peek() {
                Some(c) if is_decimal_digit(c) => self.lex_param(),
                Some(b'$') => self.lex_dollar_string(), // empty delimiter
                Some(c) if is_ident_start(c) => self.lex_dollar_string(),
                _ => Err(UnknownChar { unknown: b'$' }),
            }
            b'\'' => {
                if self.standard_conforming_strings {
                    self.lex_quote_string(BasicString, concatenable_string)
                }
                else {
                    self.lex_extended_string(concatenable_string)
                }
            }
            b'"' => self.lex_quote_ident(QuotedIdentifier),
            b'b' | b'B' | b'x' | b'X' => {
                if self.buffer.consume(b'\'') {
                    return self.lex_binary_string(BitString)
                }
                self.buffer.push_back();
                self.lex_identifier()
            }
            b'e' | b'E' => {
                if self.buffer.consume(b'\'') {
                    return self.lex_extended_string(false)
                }
                self.buffer.push_back();
                self.lex_identifier()
            }
            b'n' | b'N' => {
                // TODO: is there a need to check for nchar availability?
                // https://github.com/postgres/postgres/blob/1d80d6b50e6401828fc445151375f9bde3f99ac6/src/backend/parser/scan.l#L539
                if self.buffer.consume(b'\'') {
                    return if self.standard_conforming_strings {
                        self.lex_quote_string(NationalString, false)
                    }
                    else {
                        self.lex_extended_string(false)
                    }
                }
                self.buffer.push_back();
                self.lex_identifier()
            }
            b'u' | b'U' => {
                if self.buffer.consume(b'&') {
                    match self.buffer.peek() {
                        Some(b'\'') => { // u&'...'
                            if !self.standard_conforming_strings {
                                return Err(UnsafeUnicodeString)
                            }
                            self.buffer.advance_char();
                            self.lex_quote_string(UnicodeString, false)
                        }
                        Some(b'"') => { // u&"..."
                            self.buffer.advance_char();
                            self.lex_quote_ident(UnicodeIdentifier)
                        }
                        _ => {
                            self.buffer.push_back(); // push back '&'
                            self.lex_identifier() // identifier starting with 'u'/'U'
                        }
                    }
                }
                else {
                    self.lex_identifier() // identifier starting with u/U
                }
            }
            b'0' => match self.buffer.peek() {
                None => Ok(NumberLiteral { radix: 10 }),
                Some(c) => match c {
                    b'x' | b'X' => self.lex_hex_integer(),
                    b'o' | b'O' => self.lex_oct_integer(),
                    b'b' | b'B' => self.lex_bin_integer(),
                    _ => self.lex_dec_integer(),
                }
            }
            b'1'..=b'9' => self.lex_dec_integer(),
            op if is_op(op) => self.lex_operator(),
            id if is_ident_start(id) => self.lex_identifier(),
            unknown => Err(UnknownChar { unknown }),
        }
    }

    #[inline] // it's only used in 1 place
    fn lex_operator(&mut self) -> LexResult {

        self.buffer.push_back(); // so it's easier to consume

        // All trivia have already been consumed, so it never starts as a comment ("/*" or "--").
        // The length is guaranteed to be at least 1.

        let start_pos = self.buffer.current_index();
        let mut pg_op = false;
        while self.buffer.peek().is_some_and(is_op) {
            if self.buffer.lookahead(b"--") || self.buffer.lookahead(b"/*") {
                // This condition never happens for the 1st char,
                // because trivia have already been consumed.
                break
            }

            // Consume all ops for now, and check for restrictions afterward
            let c = self.buffer.advance_char().unwrap();
            pg_op |= is_pg_op(c)
        }

        // Length is guaranteed to be at least 1,
        // so it's safe to unwrap,
        // even though there's a push_back.
        let mut op = TokenSpan::new(self, start_pos).unwrap().slice();

        match op {
            b"%"  => Ok(Percent),
            b"*"  => Ok(Mul),
            b"+"  => Ok(Plus),
            b"-"  => Ok(Minus),
            b"/"  => Ok(Div),
            b"<"  => Ok(Less),
            b"="  => Ok(Equals),
            b">"  => Ok(Greater),
            b"^"  => Ok(Circumflex),
            b"=>" => Ok(EqualsGreater),
            b"<=" => Ok(LessEquals),
            b">=" => Ok(GreaterEquals),
            b"!=" => Ok(NotEquals),
            b"<>" => Ok(NotEquals),
            _ => {
                // Custom operator with PG op chars can have '+' or '-' as suffixes.
                // E.g., '?-' is a valid operator.

                if !pg_op {
                    // Custom operators that only have SQL-standard chars
                    // cannot have '+' or '-' as suffixes.
                    // E.g., '=-' should be tokenized as '=' and '-' separately.
                    while let Some(b'+' | b'-') = op.last() {
                        op = &op[..(op.len() - 1)];
                        self.buffer.push_back()
                    }
                }

                if op.len() >= NAMEDATALEN {
                    Err(OperatorTooLong)
                }
                else {
                    Ok(UserDefinedOperator)
                }
            }
        }
    }

    #[inline] // it's only used in 1 place
    fn lex_param(&mut self) -> LexResult {

        // $ has already been consumed, so no need to worry about it here

        let start_pos = self.buffer.current_index();

        self.buffer.consume_while(is_decimal_digit);

        // check junk
        let consumed = self.buffer.consume_if(is_ident_start);
        if consumed {
            return Err(TrailingJunkAfterParameter)
        }

        let span = TokenSpan::new(self, start_pos).unwrap();

        if span.range().len() >= 10 && span.slice()[0] > b'2' {
            // Careful with leading 0's.
            // Fail fast:
            //   The leading digit in i32::MAX is '2',
            //   so if the leading digit is above,
            //   then the string can't be safely parsed as an i32.
            return Err(ParameterNumberTooLarge)
        }

        let slice = span.slice();
        let param = slice.iter()
            .map(|d| (d - b'0') as i32)
            .try_fold(0i32, |acc, n|
                acc.checked_mul(10)?.checked_add(n)
            );

        if let Some(index) = param {
            Ok(Param { index })
        }
        else {
            Err(ParameterNumberTooLarge)
        }
    }

    #[inline] // it's only used in 1 place
    fn lex_dec_float(&mut self) -> LexResult {

        // \. {dec_digits} {dec_real}

        // The first char is '.' and it has already been consumed.
        // It's already known that the first dot is followed by a digit,
        // so there's no need to do any check here
        self.lex_dec_digits();
        self.lex_dec_real()?;
        Ok(NumberLiteral { radix: 10 })
    }

    fn lex_dec_integer(&mut self) -> LexResult {

        //   {dec_digits} (?= \.\. <dot_dot>)
        // | {dec_digits} (\. {dec_digits}?)? R

        // It's easier if the digit is included.
        self.buffer.push_back();
        self.lex_dec_digits();

        if self.buffer.consume(b'.') {
            if self.buffer.peek().is_some_and(|c| c == b'.') {
                // Don't consume '..' now.
                // It'll get consumed later as DotDot.
                self.buffer.push_back();
                return Ok(NumberLiteral { radix: 10 })
            }
            // A trailing \. is valid, so this match is optional (empty match).
            self.lex_dec_digits();
        }

        self.lex_dec_real()?;

        Ok(NumberLiteral { radix: 10 })
    }

    fn lex_dec_real(&mut self) -> Result<(), LexerError> {

        // Returns:
        //   Ok(true)  - When the pattern matched successfully after '[Ee]'.
        //   Ok(false) - When the pattern didn't match (empty match).
        //   Err(junk) - When there's a trailing {ident_start}.

        /*
            (
                [Ee] ([-+] (?!\d <!junk>))? D
            )? (
                {ident_start} <!junk>
            )?
        */

        if self.buffer.consume_if(|c| c == b'E' || c == b'e') {
            let sign = self.buffer.consume_if(|c| c == b'+' || c == b'-');
            let dec = self.lex_dec_digits();
            if !dec {
                if sign {
                    // [Ee] [+-] (?!\d)
                    return Err(TrailingJunkAfterNumericLiteral)
                }
                // [Ee] (?![+-\d])
                self.buffer.push_back();
            }
        }

        if self.buffer.peek().is_some_and(is_ident_start) {
            return Err(TrailingJunkAfterNumericLiteral)
        }

        Ok(())
    }

    fn lex_dec_digits(&mut self) -> bool {

        // \d+ ( _? \d+ )*

        let mut consumed = self.buffer.consume_while(is_decimal_digit);
        if consumed == 0 {
            return false
        }

        while consumed > 0 {
            let underscore = self.buffer.consume(b'_');
            consumed = self.buffer.consume_while(is_decimal_digit);

            if consumed == 0 && underscore {
                self.buffer.push_back()
            }
        }

        true
    }

    #[inline(always)]
    fn lex_hex_integer(&mut self) -> LexResult {
        self.lex_prefixed_int(is_hex_digit, 16)
    }

    #[inline(always)]
    fn lex_oct_integer(&mut self) -> LexResult {
        self.lex_prefixed_int(is_oct_digit, 8)
    }

    #[inline(always)]
    fn lex_bin_integer(&mut self) -> LexResult {
        self.lex_prefixed_int(is_bin_digit, 2)
    }

    fn lex_prefixed_int(&mut self, is_digit: impl Fn(u8) -> bool, radix: i32) -> LexResult {
        self.buffer.advance_char(); // ignore [xXoObB]

        let start_pos = self.buffer.current_index();

        // /(_?{digit}+)*/
        let mut consumed = usize::MAX;
        while consumed > 0 {
            self.buffer.consume(b'_');
            consumed = self.buffer.consume_while(&is_digit);
        }

        let end_pos = self.buffer.current_index();
        let span = self.buffer.source();
        let span = &span[start_pos..end_pos];

        if span.is_empty() || span.last().is_some_and(|c| *c == b'_') {
            return Err(InvalidInteger { radix })
        }

        if self.buffer.peek().is_some_and(is_ident_start) {
            return Err(TrailingJunkAfterNumericLiteral)
        }

        Ok(NumberLiteral { radix })
    }

    #[inline] // it's only used in 1 place
    fn lex_binary_string(&mut self, kind: StringKind) -> LexResult {

        // No content validation to simplify the lexer.

        loop {
            match self.buffer.advance_char() {
                None => return Err(UnterminatedString),
                Some(b'\'') => return Ok(StringLiteral { kind, concatenable: false }),
                _ => {}
            }
        }
    }

    fn lex_quote_ident(&mut self, ident_kind: IdentifierKind) -> LexResult {

        let start_pos = self.buffer.current_index();

        loop {
            match self.buffer.advance_char() {
                None => return Err(UnterminatedQuotedIdentifier),
                Some(b'"') => {
                    if let Some(b'"') = self.buffer.peek() {
                        // escaped double quote '""'
                        self.buffer.advance_char();
                    } else {
                        return if self.buffer.current_index() - start_pos == 1 {
                            Err(EmptyDelimitedIdentifier) // only consumed '"'
                        } else {
                            Ok(Identifier(ident_kind))
                        }
                    }
                }
                _ => {} // consume the char and continue
            }
        }
    }

    fn lex_quote_string(&mut self, kind: StringKind, concatenable: bool) -> LexResult {

        loop {
            match self.buffer.advance_char() {
                None => return Err(UnterminatedString),
                Some(b'\'') => {
                    if let Some(b'\'') = self.buffer.peek() {
                        self.buffer.advance_char();
                    } else {
                        return Ok(StringLiteral { kind, concatenable })
                    }
                }
                _ => {} // consume the char and continue
            }
        }
    }

    fn lex_extended_string(&mut self, concatenable: bool) -> LexResult {

        // To keep the lexer simple, parsing escapes will be done at a later point.
        // This way the lexer doesn't need to work with Strings,
        // or have separate validation and parsing phases.

        loop {
            match self.buffer.advance_char() {
                None => return Err(UnterminatedString),
                Some(b'\\') => {
                    if self.buffer.advance_char().is_none() {
                        return Err(UnterminatedString);
                    }
                }
                Some(b'\'') => {
                    if let Some(b'\'') = self.buffer.peek() {
                        self.buffer.advance_char();
                    } else {
                        return Ok(StringLiteral { kind: ExtendedString, concatenable })
                    }
                }
                _ => {} // consume the char and continue
            }
        }
    }

    #[inline]
    fn lex_identifier(&mut self) -> LexResult {

        // {ident_start} was already consumed

        self.buffer.consume_while(is_ident_cont);
        Ok(Identifier(BasicIdentifier))
    }

    fn lex_dollar_string(&mut self) -> LexResult {

        // The delimiter always contains '$' as the last char,
        // even if the delimiter is empty (i.e., '$$'),
        // so it's easier to match and consume.
        let delim = match self.lex_dollar_delim() {
            None => return Err(UnknownChar { unknown: b'$' }),
            Some(d) => d
        };

        loop {
            if self.buffer.eof() {
                return Err(UnterminatedString)
            }
            if self.buffer.consume(b'$') {
                if self.buffer.consume_string(delim) {
                    return Ok(StringLiteral { kind: DollarString, concatenable: false });
                }
                continue // $ was already consumed
            }
            self.buffer.advance_char();
        }
    }

    #[inline] // it's only used in 1 place
    fn lex_dollar_delim(&mut self) -> Option<&'source [u8]> {

        // If we're here, then the 1st char is `is_ident_start` or '$' (empty delimiter)

        let start_pos = self.buffer.current_index();

        if self.buffer.consume(b'$') {
            // Empty delimiter
            let span = TokenSpan::new(self, start_pos).unwrap();
            return Some(span.slice())
        }

        if self.buffer.consume_if(is_ident_start) {
            self.buffer.consume_while(is_dollar_quote_cont);
        }

        if !self.buffer.consume(b'$') {
            // This is the only time the lexer needs to backtrack many chars.
            self.buffer.seek(start_pos);
            return None
        }

        let span = TokenSpan::new(self, start_pos).unwrap();
        Some(span.slice())
    }

    #[inline] // it's only used in 1 place
    fn skip_trivia(&mut self) -> Result<bool, LocatableError> {

        // Postgres:
        //   Returns Ok(true) if the whitespace contains \n and no block comments.
        //   https://github.com/postgres/postgres/blob/1d80d6b50e6401828fc445151375f9bde3f99ac6/src/backend/parser/scan.l#L244

        // SQL Standard:
        //   Returns Ok(true) if the whitespace contains \n.
        //   https://sql-99.readthedocs.io/en/latest/chapters/02.html#separator
        //   TODO: allow this behaviour based on a compatibility flag, or fork the project.

        if self.buffer.eof() {
            return Ok(false)
        }

        let start_pos = self.buffer.current_index();

        let mut block_comment = false;
        loop {
            let consumed = self.buffer.consume_while(is_whitespace);
            if consumed > 0 {
                continue
            }

            if self.skip_comment() {
                continue
            }

            let consumed = self.skip_block_comment()?;
            if consumed {
                block_comment = true;
                continue
            }

            break
        }

        let (start_line, _) = self.buffer.location_at(start_pos);
        let (end_line, _) = self.buffer.location();

        Ok(!block_comment && start_line != end_line)
    }

    #[inline] // it's only used in 1 place
    fn skip_comment(&mut self) -> bool {

        if !self.buffer.consume_string(b"--") {
            return false
        }

        while let Some(c) = self.buffer.advance_char() {
            if is_new_line(c) {
                break
            }
        }

        true
    }

    fn skip_block_comment(&mut self) -> Result<bool, LocatableError> {

        let start_pos = self.buffer.current_index();

        if !self.buffer.consume_string(b"/*") {
            return Ok(false)
        }

        loop {
            if self.buffer.lookahead(b"/*") {
                self.skip_block_comment()?;
                continue
            }

            if self.buffer.consume_string(b"*/") {
                return Ok(true)
            }

            if self.buffer.eof() {
                let err = TokenSpan::new(self, start_pos)
                    .unwrap()
                    .location()
                    .of(UnterminatedBlockComment);
                return Err(err)
            }

            self.buffer.advance_char();
        }
    }
}

fn to_hex_digit_unsafe(c: &u8) -> i32 {

    let n = *c as i32;

    if n <= b'A' as i32 {
        return n - b'0' as i32
    }

    // The bit trick essentially changes uppercase to lowercase, then subtracts by 87.
    // This correctly gives b'A'..b'F' -> 10..15:
    // * 'A' - 87 = 10
    // * 'F' - 87 = 15

    // The difference between lowercase and uppercase is only in bit 6: 2**6 == 32 == 0x20

    (n | 0x20) - 87
}

fn parse_int_unsafe(
    span: &[u8],
    u8_to_i32: impl Fn(&u8) -> i32,
    radix: i32
) -> Option<i32>
{
    // Assumes the span only contains '_' or digits (dec, oct, hex)

    // try parse to i32 and return IntLiteral
    span.iter()
        .filter_map(|c|
            if *c != b'_' {
                Some(u8_to_i32(c))
            }
            else {
                None
            }
        ) // ignore underscores
        .try_fold(0i32, |acc, n|
            acc.checked_mul(radix)?.checked_add(n)
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range;

    #[test]
    fn test_empty_string() {
        let source = b"";
        let mut lex = Lexer::new(source, true);

        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_whitespace() {
        let source = b"\t\r\x0b\x0c\n \x0b\t\r\n \x0c\r\x0b\x0c \n\t";
        let mut lex = Lexer::new(source, true);

        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_unknown_char() {
        let source = b"\x00";
        let mut lex = Lexer::new(source, true);

        assert_eq!(err(UnknownChar { unknown: b'\x00' }, 0..1, 1, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_operators() {
        let source = b". .. ( ) , ; [ ] : :: := % * + - / < = > ^ => <= >= != <>";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(Dot, 0..1, 1, 1), lex.next());
        assert_eq!(tok(DotDot, 2..4, 1, 3), lex.next());
        assert_eq!(tok(OpenParenthesis, 5..6, 1, 6), lex.next());
        assert_eq!(tok(CloseParenthesis, 7..8, 1, 8), lex.next());
        assert_eq!(tok(Comma, 9..10, 1, 10), lex.next());
        assert_eq!(tok(Semicolon, 11..12, 1, 12), lex.next());
        assert_eq!(tok(OpenBracket, 13..14, 1, 14), lex.next());
        assert_eq!(tok(CloseBracket, 15..16, 1, 16), lex.next());
        assert_eq!(tok(Colon, 17..18, 1, 18), lex.next());
        assert_eq!(tok(Typecast, 19..21, 1, 20), lex.next());
        assert_eq!(tok(ColonEquals, 22..24, 1, 23), lex.next());
        assert_eq!(tok(Percent, 25..26, 1, 26), lex.next());
        assert_eq!(tok(Mul, 27..28, 1, 28), lex.next());
        assert_eq!(tok(Plus, 29..30, 1, 30), lex.next());
        assert_eq!(tok(Minus, 31..32, 1, 32), lex.next());
        assert_eq!(tok(Div, 33..34, 1, 34), lex.next());
        assert_eq!(tok(Less, 35..36, 1, 36), lex.next());
        assert_eq!(tok(Equals, 37..38, 1, 38), lex.next());
        assert_eq!(tok(Greater, 39..40, 1, 40), lex.next());
        assert_eq!(tok(Circumflex, 41..42, 1, 42), lex.next());
        assert_eq!(tok(EqualsGreater, 43..45, 1, 44), lex.next());
        assert_eq!(tok(LessEquals, 46..48, 1, 47), lex.next());
        assert_eq!(tok(GreaterEquals, 49..51, 1, 50), lex.next());
        assert_eq!(tok(NotEquals, 52..54, 1, 53), lex.next());
        assert_eq!(tok(NotEquals, 55..57, 1, 56), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_user_defined_operators() {
        let source = b"\
        //=-\n\
        -@-\n\
        ";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(UserDefinedOperator, 0..3, 1, 1), lex.next());
        assert_eq!(tok(Minus, 3..4, 1, 4), lex.next());
        assert_eq!(tok(UserDefinedOperator, 5..8, 2, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_param() {
        let source = b"$0123";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(Param { index: 123 }, 0..5, 1, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_hex_number() {
        let source = b"0x_1_C0e_E_a92";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(NumberLiteral { radix: 16 }, 0..14, 1, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_oct_number() {
        let source = b"0o20155_53_7";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(NumberLiteral { radix: 8 }, 0..12, 1, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_bin_number() {
        let source = b"0b1_001000_01001_01101";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(NumberLiteral { radix: 2 }, 0..22, 1, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_integer() {
        let source = b"\
        0_010\n\
        9_8_7_6\n\
        0\
        ";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(NumberLiteral { radix: 10 }, 0..5, 1, 1), lex.next());
        assert_eq!(tok(NumberLiteral { radix: 10 }, 6..13, 2, 1), lex.next());
        assert_eq!(tok(NumberLiteral { radix: 10 }, 14..15, 3, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_integer_dot_dot() {
        let source = b"184..";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(NumberLiteral { radix: 10 }, 0..3, 1, 1), lex.next());
        assert_eq!(tok(DotDot, 3..5, 1, 4), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_float() {
        let source = b"\
        .01_23e-043_5_00\n\
        475.\n\
        1.1\
        ";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(NumberLiteral { radix: 10 }, 0..16, 1, 1), lex.next());
        assert_eq!(tok(NumberLiteral { radix: 10 }, 17..21, 2, 1), lex.next());
        assert_eq!(tok(NumberLiteral { radix: 10 }, 22..25, 3, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string_with_empty_delim() {
        let source = b"$$some string$$";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(StringLiteral { kind: DollarString, concatenable: false }, 0..15, 1, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string() {
        let source = b"$foo$bar baz$foo$";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(StringLiteral { kind: DollarString, concatenable: false }, 0..17, 1, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string_with_dollars() {
        let source = b"$foo$dolla $ dolla $$ bill$$foo$";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(StringLiteral { kind: DollarString, concatenable: false }, 0..32, 1, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string_mismatch() {
        let source = b"$not a string";
        let mut lex = Lexer::new(source, true);

        assert_eq!(err(UnknownChar { unknown: b'$' }, 0..1, 1, 1), lex.next());
        assert_eq!(tok(Identifier(BasicIdentifier), 1..4, 1, 2), lex.next());
        assert_eq!(tok(Identifier(BasicIdentifier), 5..6, 1, 6), lex.next());
        assert_eq!(tok(Identifier(BasicIdentifier), 7..13, 1, 8), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_quote_string() {
        let source = b"\
        ''\n\
        'concatenable' '\\'''\n\
        N'national'\
        ";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(StringLiteral { kind: BasicString, concatenable: false }, 0..2, 1, 1), lex.next());
        assert_eq!(tok(StringLiteral { kind: BasicString, concatenable: true }, 3..17, 2, 1), lex.next());
        assert_eq!(tok(StringLiteral { kind: BasicString, concatenable: false }, 18..23, 2, 16), lex.next());
        assert_eq!(tok(StringLiteral { kind: NationalString, concatenable: false }, 24..35, 3, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_escape_strings() {
        let source = b"\
        '''quotes\\''\n\
        e'''quotes\\''\n\
        n'national'\
        ";
        let mut lex = Lexer::new(source, false);

        assert_eq!(tok(StringLiteral { kind: ExtendedString, concatenable: false }, 0..12, 1, 1), lex.next());
        assert_eq!(tok(StringLiteral { kind: ExtendedString, concatenable: false }, 13..26, 2, 1), lex.next());
        assert_eq!(tok(StringLiteral { kind: ExtendedString, concatenable: false }, 27..38, 3, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_bit_string() {
        let source = b"b'0_156e_wf' x'048_96a_f_d'"; // lexer doesn't validate chars
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(StringLiteral { kind: BitString, concatenable: false }, 0..12, 1, 1), lex.next());
        assert_eq!(tok(StringLiteral { kind: BitString, concatenable: false }, 13..27, 1, 14), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_unicode_string() {
        let source = b"\
        u&''\n\
        U&'unicode\\'\
        ";
        let mut lex = Lexer::new(source, true);

        assert_eq!(tok(StringLiteral { kind: UnicodeString, concatenable: false }, 0..4, 1, 1), lex.next());
        assert_eq!(tok(StringLiteral { kind: UnicodeString, concatenable: false }, 5..17, 2, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_identifier() {
        let source = b"bar xyz efg nun ube foo u&x";
        let mut lex = Lexer::new(source, true);
        assert_eq!(tok(Identifier(BasicIdentifier), 0..3, 1, 1), lex.next());
        assert_eq!(tok(Identifier(BasicIdentifier), 4..7, 1, 5), lex.next());
        assert_eq!(tok(Identifier(BasicIdentifier), 8..11, 1, 9), lex.next());
        assert_eq!(tok(Identifier(BasicIdentifier), 12..15, 1, 13), lex.next());
        assert_eq!(tok(Identifier(BasicIdentifier), 16..19, 1, 17), lex.next());
        assert_eq!(tok(Identifier(BasicIdentifier), 20..23, 1, 21), lex.next());
        assert_eq!(tok(Identifier(BasicIdentifier), 24..25, 1, 25), lex.next());
        assert_eq!(tok(UserDefinedOperator, 25..26, 1, 26), lex.next());
        assert_eq!(tok(Identifier(BasicIdentifier), 26..27, 1, 27), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_quote_ident() {
        let source = b"\
        \"\"\n\
        \"\"\"escaped\"\n\
        u&\"uni\"\"code\"\
        ";
        let mut lex = Lexer::new(source, true);

        assert_eq!(err(EmptyDelimitedIdentifier, 0..2, 1, 1), lex.next());
        assert_eq!(tok(Identifier(QuotedIdentifier), 3..14, 2, 1), lex.next());
        assert_eq!(tok(Identifier(UnicodeIdentifier), 15..28, 3, 1), lex.next());
        assert_eq!(None, lex.next());
    }

    fn tok(kind: TokenKind, range: Range<usize>, line: usize, col: usize) -> Option<LexerResult> {
        Some(Ok(
            Location::new(range, line, col).of(kind)
        ))
    }

    fn err(err: LexerError, range: Range<usize>, line: usize, col: usize) -> Option<LexerResult> {
        Some(Err(
            Location::new(range, line, col).of(err)
        ))
    }
}
