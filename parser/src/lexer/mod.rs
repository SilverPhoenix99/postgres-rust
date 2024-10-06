mod keyword;
mod error;
mod token_kind;

pub use self::{
    error::{LexerError, LexerErrorKind},
    keyword::{Keyword, KeywordDetails},
    token_kind::{BitStringKind, IdentifierKind, StringKind, TokenKind},
};

pub type LexerResult = Result<Located<TokenKind>, LexerError>;
type LexResult<T = TokenKind> = Result<T, (LexerErrorKind, FnInfo)>;

#[derive(Debug)]
pub struct Lexer<'src> {
    buffer: CharBuffer<'src>,
    standard_conforming_strings: bool
}

impl Iterator for Lexer<'_> {
    type Item = LexerResult;

    /// The token is always a full match,
    /// never a substring that's more interesting than the whole match.
    fn next(&mut self) -> Option<Self::Item> {

        let concatenable_whitespace = match self.skip_trivia() {
            Ok(concatenable_whitespace) => concatenable_whitespace,
            Err(err) => {
                return Some(Err(err))
            }
        };

        if self.buffer.eof() {
            return None
        }

        let start_index = self.buffer.current_index();
        let token = self.lex_token(concatenable_whitespace);
        let location = self.buffer.location_starting_at(start_index);

        match token {
            Ok(kind) => Some(Ok((kind, location))),
            Err((err_code, fninfo)) => {
                let report = LexerError::new(err_code, fninfo, location);
                Some(Err(report))
            },
        }
    }
}

impl FusedIterator for Lexer<'_> {}

impl<'src> Lexer<'src> {

    #[inline]
    pub fn new(source: &'src str, standard_conforming_strings: bool) -> Self {
        Self {
            buffer: CharBuffer::new(source),
            standard_conforming_strings
        }
    }

    #[inline(always)]
    pub fn source(&self) -> &'src str {
        self.buffer.source()
    }

    /// Zero-length `range`.
    #[inline(always)]
    pub fn current_location(&self) -> Location {
        self.buffer.current_location()
    }

    #[inline(always)]
    pub fn eof(&self) -> bool {
        self.buffer.eof()
    }

    fn lex_token(&mut self, concatenable_string: bool) -> LexResult {

        // eof has already been filtered
        match self.buffer.consume_one().unwrap() {
            '(' => Ok(OpenParenthesis),
            ')' => Ok(CloseParenthesis),
            ',' => Ok(Comma),
            ';' => Ok(Semicolon),
            '[' => Ok(OpenBracket),
            ']' => Ok(CloseBracket),
            '.' => {
                if self.buffer.consume_char('.') {
                    Ok(DotDot)
                }
                else if self.buffer.peek().is_some_and(is_decimal_digit) {
                    self.lex_dec_float()
                }
                else {
                    Ok(Dot)
                }
            }
            ':' => {
                if self.buffer.consume_char(':') {
                    Ok(Typecast)
                }
                else if self.buffer.consume_char('=') {
                    Ok(ColonEquals)
                }
                else {
                    Ok(Colon)
                }
            }
            '$' => match self.buffer.peek() {
                Some(c) if is_decimal_digit(c) => self.lex_param(),
                Some('$') => self.lex_dollar_string(), // empty delimiter
                Some(c) if is_ident_start(c) => self.lex_dollar_string(),
                _ => Err((UnexpectedChar { unknown: '$' }, fn_info!())),
            }
            '\'' => {
                if self.standard_conforming_strings {
                    self.lex_quote_string(BasicString { concatenable: concatenable_string })
                }
                else {
                    self.lex_extended_string(concatenable_string)
                }
            }
            '"' => self.lex_quote_ident(QuotedIdentifier),
            'b' | 'B' => {
                if self.buffer.consume_char('\'') {
                    return self.lex_bit_string(BinaryString)
                }
                self.lex_identifier()
            }
            'x' | 'X' => {
                if self.buffer.consume_char('\'') {
                    return self.lex_bit_string(HexString)
                }
                self.lex_identifier()
            }
            'e' | 'E' => {
                if self.buffer.consume_char('\'') {
                    return self.lex_extended_string(false)
                }
                self.lex_identifier()
            }
            'n' | 'N' => {
                // TODO: is there a need to check for nchar availability?
                // https://github.com/postgres/postgres/blob/1d80d6b50e6401828fc445151375f9bde3f99ac6/src/backend/parser/scan.l#L539
                if self.buffer.consume_char('\'') {
                    let tok = if self.standard_conforming_strings {
                        self.lex_quote_string(NationalString)
                    }
                    else {
                        self.lex_extended_string(false)
                    };
                    return tok
                }
                self.lex_identifier()
            }
            'u' | 'U' => {
                if self.buffer.consume_char('&') {
                    match self.buffer.peek() {
                        Some('\'') => { // u&'...'
                            if !self.standard_conforming_strings {
                                return Err((UnsafeUnicodeString, fn_info!()))
                            }
                            self.buffer.consume_one();
                            self.lex_quote_string(UnicodeString)
                        }
                        Some('"') => { // u&"..."
                            self.buffer.consume_one();
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
            '0' => match self.buffer.peek() {
                None => Ok(NumberLiteral { radix: 10 }),
                Some(c) => match c {
                    'x' | 'X' => self.lex_hex_integer(),
                    'o' | 'O' => self.lex_oct_integer(),
                    'b' | 'B' => self.lex_bin_integer(),
                    _ => self.lex_dec_integer(),
                }
            }
            '1'..='9' => self.lex_dec_integer(),
            op if is_op(op) => self.lex_operator(),
            id if is_ident_start(id) => self.lex_identifier(),
            unknown => {
                Err((UnexpectedChar { unknown }, fn_info!()))
            },
        }
    }

    #[inline] // Only called from a single place
    fn lex_operator(&mut self) -> LexResult {

        self.buffer.push_back(); // so it's easier to consume

        // All trivia have already been consumed, so it never starts as a comment ("/*" or "--").
        // The length is guaranteed to be at least 1.

        let start_index = self.buffer.current_index();
        let mut pg_op = false;
        while self.buffer.peek().is_some_and(is_op) {
            let is_comment_start = {
                let rem = self.buffer.remainder();
                rem.starts_with("--") || rem.starts_with("/*")
            };
            if is_comment_start {
                // This condition never happens for the 1st char,
                // because trivia have already been consumed.
                break
            }

            // Consume all ops for now, and check for restrictions afterward
            let c = self.buffer.consume_one().unwrap();
            pg_op |= is_pg_op(c)
        }

        // Length is guaranteed to be at least 1,
        // so it's safe to unwrap,
        // even though there's a push_back.
        let mut op = self.buffer.slice(start_index);

        match op {
            "%"  => Ok(Percent),
            "*"  => Ok(Mul),
            "+"  => Ok(Plus),
            "-"  => Ok(Minus),
            "/"  => Ok(Div),
            "<"  => Ok(Less),
            "="  => Ok(Equals),
            ">"  => Ok(Greater),
            "^"  => Ok(Circumflex),
            "=>" => Ok(EqualsGreater),
            "<=" => Ok(LessEquals),
            ">=" => Ok(GreaterEquals),
            "!=" => Ok(NotEquals),
            "<>" => Ok(NotEquals),
            _ => {
                // Custom operator with PG op chars can have '+' or '-' as suffixes.
                // E.g., '?-' is a valid operator.

                if !pg_op {
                    // Custom operators that only have SQL-standard chars
                    // cannot have '+' or '-' as suffixes.
                    // E.g., '=-' should be tokenized as '=' and '-' separately.
                    let num = op.as_bytes()
                        .iter()
                        .rev()
                        .take_while(|c| **c == b'+' || **c == b'-')
                        .count();
                    // SAFETY: only returns ASCII chars ('+' and '-')
                    self.buffer.seek(self.buffer.current_index() - num);

                    let len = op.len() - num;
                    op = &op[..len];
                }

                if op.len() >= NAMEDATALEN {
                    Err((OperatorTooLong, fn_info!()))
                }
                else {
                    Ok(UserDefinedOperator)
                }
            }
        }
    }

    #[inline] // Only called from a single place
    fn lex_param(&mut self) -> LexResult {

        // $ has already been consumed, so no need to worry about it here

        let start_index = self.buffer.current_index();

        self.buffer.consume_while(is_decimal_digit);

        // check junk
        let consumed = self.buffer.consume_if(is_ident_start);
        if consumed.is_some() {
            return Err((TrailingJunkAfterParameter, fn_info!()))
        }

        // SAFETY: They're all ASCII decimal digits
        let slice = self.buffer.slice(start_index).as_bytes();

        if slice.len() >= 10 && slice[0] > b'2' {
            // Careful with leading 0's.
            // Fail fast:
            //   The leading digit in i32::MAX is '2',
            //   so if the leading digit is above,
            //   then the string can't be safely parsed as an i32.
            return Err((ParameterNumberTooLarge, fn_info!()))
        }

        // i32 is used to match original PG's expectation that it won't be > i32::MAX
        slice.iter()
            .map(|d| (d - b'0') as i32)
            .try_fold(0i32, |acc, n|
                acc.checked_mul(10)?.checked_add(n)
            )
            .map_or(
                Err((ParameterNumberTooLarge, fn_info!())),
                |index| Ok(Param { index })
            )
    }

    #[inline] // Only called from a single place
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

        if self.buffer.consume_char('.') {
            if self.buffer.peek().is_some_and(|c| c == '.') {
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

    fn lex_dec_real(&mut self) -> LexResult<()> {

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

        let exp = self.buffer.consume_if(|c| c == 'E' || c == 'e');
        if exp.is_some() {
            let sign = self.buffer.consume_if(|c| c == '+' || c == '-')
                .is_some();
            let dec = self.lex_dec_digits();
            if !dec {
                if sign {
                    // [Ee] [+-] (?!\d)
                    return Err((TrailingJunkAfterNumericLiteral, fn_info!()))
                }
                // [Ee] (?![+-\d])
                self.buffer.push_back();
            }
        }

        if self.buffer.peek().is_some_and(is_ident_start) {
            return Err((TrailingJunkAfterNumericLiteral, fn_info!()))
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
            let underscore = self.buffer.consume_char('_');
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

    fn lex_prefixed_int(&mut self, is_digit: impl Fn(char) -> bool, radix: u32) -> LexResult {

        self.buffer.consume_one(); // ignore [xXoObB]

        let start_index = self.buffer.current_index();

        // /(_?{digit}+)*/
        let mut consumed = usize::MAX;
        while consumed > 0 {
            self.buffer.consume_char('_');
            consumed = self.buffer.consume_while(&is_digit);
        }

        // SAFETY: They're all ASCII chars ('_', or decimal & hex digits)
        let span = self.buffer.slice(start_index).as_bytes();

        if span.is_empty() || span.last().is_some_and(|c| *c == b'_') {
            return Err((InvalidInteger { radix }, fn_info!()))
        }

        if self.buffer.peek().is_some_and(is_ident_start) {
            return Err((TrailingJunkAfterNumericLiteral, fn_info!()))
        }

        Ok(NumberLiteral { radix })
    }

    #[inline] // Only called from a single place
    fn lex_bit_string(&mut self, kind: BitStringKind) -> LexResult {

        // No content validation to simplify the lexer.

        loop {
            match self.buffer.consume_one() {
                None => {
                    let err = if kind == HexString {
                        UnterminatedHexString
                    }
                    else {
                        UnterminatedBitString
                    };
                    return Err((err, fn_info!()))
                },
                Some('\'') => return Ok(BitStringLiteral(kind)),
                _ => {}
            }
        }
    }

    fn lex_quote_ident(&mut self, ident_kind: IdentifierKind) -> LexResult {

        let start_index = self.buffer.current_index();

        loop {
            let Some(c) = self.buffer.consume_one() else {
                return Err((UnterminatedQuotedIdentifier, fn_info!()))
            };

            if c != '"' {
                continue
            }

            if let Some('"') = self.buffer.peek() {
                // escaped double quote '""'
                self.buffer.consume_one();
                continue
            }

            return if self.buffer.current_index() - start_index == 1 {
                Err((EmptyDelimitedIdentifier, fn_info!())) // only consumed '"'
            }
            else {
                Ok(Identifier(ident_kind))
            }
        }
    }

    fn lex_quote_string(&mut self, kind: StringKind) -> LexResult {

        loop {
            let Some(c) = self.buffer.consume_one() else {
                return Err((UnterminatedQuotedString, fn_info!()))
            };

            if c != '\'' {
                continue
            }

            if let Some('\'') = self.buffer.peek() {
                self.buffer.consume_one();
                continue
            }

            return Ok(StringLiteral(kind))
        }
    }

    fn lex_extended_string(&mut self, concatenable: bool) -> LexResult {

        // To keep the lexer simple, parsing escapes will be done at a later point.
        // This way the lexer doesn't need to work with Strings,
        // or have separate validation and parsing phases.

        loop {
            let Some(c) = self.buffer.consume_one() else {
                return Err((UnterminatedQuotedString, fn_info!()))
            };

            if c == '\\' && self.buffer.consume_one().is_none() {
                return Err((UnterminatedQuotedString, fn_info!()))
            }

            if c == '\'' {
                if let Some('\'') = self.buffer.peek() {
                    self.buffer.consume_one();
                }
                else {
                    return Ok(StringLiteral( ExtendedString { concatenable } ))
                }
            }
        }
    }

    #[inline]
    fn lex_identifier(&mut self) -> LexResult {

        // To prevent re-consuming it, {ident_start} was already consumed.
        let start_index = self.buffer.current_index() - 1;

        self.buffer.consume_while(is_ident_cont);

        let ident = self.buffer.slice(start_index)
            .to_ascii_lowercase();

        if let Some(kw) = KeywordDetails::find(&ident) {
            return Ok(Keyword(kw))
        }

        Ok(Identifier(BasicIdentifier))
    }

    fn lex_dollar_string(&mut self) -> LexResult {

        // The delimiter always contains '$' as the last char,
        // even if the delimiter is empty (i.e., '$$'),
        // so it's easier to match and consume.

        let Some(delim) = self.lex_dollar_delim() else {
            return Err((UnexpectedChar { unknown: '$' }, fn_info!()))
        };

        loop {
            if self.buffer.eof() {
                return Err((UnterminatedDollarQuotedString, fn_info!()))
            }
            if self.buffer.consume_char('$') {
                if self.buffer.consume_string(delim) {
                    return Ok(StringLiteral(DollarString));
                }
                continue // $ was already consumed
            }
            self.buffer.consume_one();
        }
    }

    #[inline] // Only called from a single place
    fn lex_dollar_delim(&mut self) -> Option<&'src str> {

        // If we're here, then the 1st char is `is_ident_start` or '$' (empty delimiter)

        let start_index = self.buffer.current_index();

        if self.buffer.consume_char('$') {
            // Empty delimiter
            let slice = self.buffer.slice(start_index);
            return Some(slice)
        }

        if self.buffer.consume_if(is_ident_start).is_some() {
            self.buffer.consume_while(is_dollar_quote_cont);
        }

        if !self.buffer.consume_char('$') {
            // This is the only time the lexer needs to backtrack many chars.
            // SAFETY: `start_index` was given by the buffer itself, so it's a safe and correct index to give back to it.
            self.buffer.seek(start_index);
            return None
        }

        let slice = self.buffer.slice(start_index);
        Some(slice)
    }

    #[inline] // Only called from a single place
    fn skip_trivia(&mut self) -> Result<bool, LexerError> {

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

        let start_index = self.buffer.current_index();

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

        let (start_line, _) = self.buffer.position_at(start_index);
        let (end_line, _) = self.buffer.current_position();

        Ok(!block_comment && start_line != end_line)
    }

    #[inline] // Only called from a single place
    fn skip_comment(&mut self) -> bool {

        if !self.buffer.consume_string("--") {
            return false
        }

        while let Some(c) = self.buffer.consume_one() {
            if is_new_line(c) {
                break
            }
        }

        true
    }

    fn skip_block_comment(&mut self) -> Result<bool, LexerError> {

        let start_index = self.buffer.current_index();

        if !self.buffer.consume_string("/*") {
            return Ok(false)
        }

        loop {
            if self.buffer.remainder().starts_with("/*") {
                self.skip_block_comment()?;
                continue
            }

            if self.buffer.consume_string("*/") {
                return Ok(true)
            }

            if self.buffer.eof() {
                let loc = self.buffer.location_starting_at(start_index);
                let report = LexerError::new(UnterminatedBlockComment, fn_info!(), loc);
                return Err(report)
            }

            self.buffer.consume_one();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::HasLocation;
    use crate::lexer::Keyword::{From, Not, Select, StringKw};
    use std::ops::Range;

    #[test]
    fn test_empty_string() {
        let source = "";
        let mut lex = Lexer::new(source, true);

        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_whitespace() {
        let source = "\t\r\x0b\x0c\n \x0b\t\r\n \x0c\r\x0b\x0c \n\t";
        let mut lex = Lexer::new(source, true);

        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_unknown_char() {
        let source = "\x00";
        let mut lex = Lexer::new(source, true);

        assert_err(UnexpectedChar { unknown: '\x00' }, 0..1, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_operators() {
        let source = ". .. ( ) , ; [ ] : :: := % * + - / < = > ^ => <= >= != <>";
        let mut lex = Lexer::new(source, true);

        assert_tok(Dot, 0..1, 1, 1, lex.next());
        assert_tok(DotDot, 2..4, 1, 3, lex.next());
        assert_tok(OpenParenthesis, 5..6, 1, 6, lex.next());
        assert_tok(CloseParenthesis, 7..8, 1, 8, lex.next());
        assert_tok(Comma, 9..10, 1, 10, lex.next());
        assert_tok(Semicolon, 11..12, 1, 12, lex.next());
        assert_tok(OpenBracket, 13..14, 1, 14, lex.next());
        assert_tok(CloseBracket, 15..16, 1, 16, lex.next());
        assert_tok(Colon, 17..18, 1, 18, lex.next());
        assert_tok(Typecast, 19..21, 1, 20, lex.next());
        assert_tok(ColonEquals, 22..24, 1, 23, lex.next());
        assert_tok(Percent, 25..26, 1, 26, lex.next());
        assert_tok(Mul, 27..28, 1, 28, lex.next());
        assert_tok(Plus, 29..30, 1, 30, lex.next());
        assert_tok(Minus, 31..32, 1, 32, lex.next());
        assert_tok(Div, 33..34, 1, 34, lex.next());
        assert_tok(Less, 35..36, 1, 36, lex.next());
        assert_tok(Equals, 37..38, 1, 38, lex.next());
        assert_tok(Greater, 39..40, 1, 40, lex.next());
        assert_tok(Circumflex, 41..42, 1, 42, lex.next());
        assert_tok(EqualsGreater, 43..45, 1, 44, lex.next());
        assert_tok(LessEquals, 46..48, 1, 47, lex.next());
        assert_tok(GreaterEquals, 49..51, 1, 50, lex.next());
        assert_tok(NotEquals, 52..54, 1, 53, lex.next());
        assert_tok(NotEquals, 55..57, 1, 56, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_user_defined_operators() {
        let source = "\
        //=-\n\
        -@-\n\
        ";
        let mut lex = Lexer::new(source, true);

        assert_tok(UserDefinedOperator, 0..3, 1, 1, lex.next());
        assert_tok(Minus, 3..4, 1, 4, lex.next());
        assert_tok(UserDefinedOperator, 5..8, 2, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_param() {
        let source = "$0123";
        let mut lex = Lexer::new(source, true);

        assert_tok(Param { index: 123 }, 0..5, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_hex_number() {
        let source = "0x_1_C0e_E_a92";
        let mut lex = Lexer::new(source, true);

        assert_tok(NumberLiteral { radix: 16 }, 0..14, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_oct_number() {
        let source = "0o20155_53_7";
        let mut lex = Lexer::new(source, true);

        assert_tok(NumberLiteral { radix: 8 }, 0..12, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_bin_number() {
        let source = "0b1_001000_01001_01101";
        let mut lex = Lexer::new(source, true);

        assert_tok(NumberLiteral { radix: 2 }, 0..22, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_integer() {
        let source = "\
        0_010\n\
        9_8_7_6\n\
        0\
        ";
        let mut lex = Lexer::new(source, true);

        assert_tok(NumberLiteral { radix: 10 }, 0..5, 1, 1, lex.next());
        assert_tok(NumberLiteral { radix: 10 }, 6..13, 2, 1, lex.next());
        assert_tok(NumberLiteral { radix: 10 }, 14..15, 3, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_integer_dot_dot() {
        let source = "184..";
        let mut lex = Lexer::new(source, true);

        assert_tok(NumberLiteral { radix: 10 }, 0..3, 1, 1, lex.next());
        assert_tok(DotDot, 3..5, 1, 4, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_float() {
        let source = "\
        .01_23e-043_5_00\n\
        475.\n\
        1.1\
        ";
        let mut lex = Lexer::new(source, true);

        assert_tok(NumberLiteral { radix: 10 }, 0..16, 1, 1, lex.next());
        assert_tok(NumberLiteral { radix: 10 }, 17..21, 2, 1, lex.next());
        assert_tok(NumberLiteral { radix: 10 }, 22..25, 3, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string_with_empty_delim() {
        let source = "$$some string$$";
        let mut lex = Lexer::new(source, true);

        assert_tok(StringLiteral(DollarString), 0..15, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string() {
        let source = "$foo$bar baz$foo$";
        let mut lex = Lexer::new(source, true);

        assert_tok(StringLiteral(DollarString), 0..17, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string_with_dollars() {
        let source = "$foo$dolla $ dolla $$ bill$$foo$";
        let mut lex = Lexer::new(source, true);

        assert_tok(StringLiteral(DollarString), 0..32, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string_mismatch() {
        let source = "$not a string";
        let mut lex = Lexer::new(source, true);

        assert_err(UnexpectedChar { unknown: '$' }, 0..1, 1, 1, lex.next());
        assert_kw(Not, lex.next());
        assert_tok(Identifier(BasicIdentifier), 5..6, 1, 6, lex.next());
        assert_kw(StringKw, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_quote_string() {
        let source = "\
        ''\n\
        'concatenable' '\\'''\n\
        N'national'\
        ";
        let mut lex = Lexer::new(source, true);

        assert_tok(StringLiteral(BasicString { concatenable: false }), 0..2, 1, 1, lex.next());
        assert_tok(StringLiteral(BasicString { concatenable: true }), 3..17, 2, 1, lex.next());
        assert_tok(StringLiteral(BasicString { concatenable: false }), 18..23, 2, 16, lex.next());
        assert_tok(StringLiteral(NationalString), 24..35, 3, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_escape_strings() {
        let source = "\
        '''quotes\\''\n\
        e'''quotes\\''\n\
        n'national'\
        ";
        let mut lex = Lexer::new(source, false);

        assert_tok(StringLiteral(ExtendedString { concatenable: false }), 0..12, 1, 1, lex.next());
        assert_tok(StringLiteral(ExtendedString { concatenable: false }), 13..26, 2, 1, lex.next());
        assert_tok(StringLiteral(ExtendedString { concatenable: false }), 27..38, 3, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_bit_string() {
        let source = "b'0_156e_wf' x'048_96a_f_d'"; // lexer doesn't validate chars
        let mut lex = Lexer::new(source, true);

        assert_tok(BitStringLiteral(BinaryString), 0..12, 1, 1, lex.next());
        assert_tok(BitStringLiteral(HexString), 13..27, 1, 14, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_unicode_string() {
        let source = "\
        u&''\n\
        U&'unicode\\'\
        ";
        let mut lex = Lexer::new(source, true);

        assert_tok(StringLiteral(UnicodeString), 0..4, 1, 1, lex.next());
        assert_tok(StringLiteral(UnicodeString), 5..17, 2, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_identifier() {
        let source = "bar xyz efg nun ube foo u&x";
        let mut lex = Lexer::new(source, true);
        assert_tok(Identifier(BasicIdentifier), 0..3, 1, 1, lex.next());
        assert_tok(Identifier(BasicIdentifier), 4..7, 1, 5, lex.next());
        assert_tok(Identifier(BasicIdentifier), 8..11, 1, 9, lex.next());
        assert_tok(Identifier(BasicIdentifier), 12..15, 1, 13, lex.next());
        assert_tok(Identifier(BasicIdentifier), 16..19, 1, 17, lex.next());
        assert_tok(Identifier(BasicIdentifier), 20..23, 1, 21, lex.next());
        assert_tok(Identifier(BasicIdentifier), 24..25, 1, 25, lex.next());
        assert_tok(UserDefinedOperator, 25..26, 1, 26, lex.next());
        assert_tok(Identifier(BasicIdentifier), 26..27, 1, 27, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_keyword() {
        let source = "SeLeCt FrOm";
        let mut lex = Lexer::new(source, true);
        assert_kw(Select, lex.next());
        assert_kw(From, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_quote_ident() {
        let source = "\
        \"\"\n\
        \"\"\"escaped\"\n\
        u&\"uni\"\"code\"\
        ";
        let mut lex = Lexer::new(source, true);

        assert_err(EmptyDelimitedIdentifier, 0..2, 1, 1, lex.next());
        assert_tok(Identifier(QuotedIdentifier), 3..14, 2, 1, lex.next());
        assert_tok(Identifier(UnicodeIdentifier), 15..28, 3, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    fn assert_tok(
        expected_kind: TokenKind,
        range: Range<usize>,
        line: usize,
        col: usize,
        actual: Option<LexerResult>
    ) {
        let expected_loc = Location::new(range, line, col);
        let expected = (expected_kind, expected_loc);

        assert_matches!(
            actual,
            Some(Ok(res)) if res == expected,
            "expected {expected:?} but got {actual:?}"
        );
    }

    fn assert_err(
        expected_err: LexerErrorKind,
        range: Range<usize>,
        line: usize,
        col: usize,
        actual: Option<LexerResult>
    ) {
        let expected_loc = Location::new(range, line, col);

        assert_matches!(actual, Some(Err(err)) if err.source() == expected_err && expected_loc.eq(err.location()));
    }

    fn assert_kw(expected: Keyword, actual: Option<LexerResult>) {

        let (actual, _) = actual
            .expect("should have been Some(Ok(_))")
            .expect("should have been Ok((Keyword(_), _))");

        assert_matches!(actual, Keyword(kw) if expected == kw.keyword())
    }
}

use self::{
    error::LexerErrorKind::*,
    token_kind::{BitStringKind::*, IdentifierKind::*, StringKind::*, TokenKind::*},
};
use postgres_basics::{
    ascii::*,
    fn_info,
    CharBuffer,
    FnInfo,
    Located,
    Location,
    NAMEDATALEN,
};
use std::iter::FusedIterator;
