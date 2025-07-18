#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod keyword;
mod token_kind;

pub use self::{
    keyword::{Keyword, KeywordCategory, KeywordDetails},
    token_kind::{
        BitStringKind,
        IdentifierKind,
        OperatorKind,
        RawTokenKind,
        StringKind,
    },
};

pub(crate) type LocatedResult = lexer::LocatedResult<Located<RawTokenKind>>;
type Result<T = RawTokenKind> = lexer::Result<T>;

#[derive(Debug)]
pub struct Lexer<'src> {
    standard_conforming_strings: bool,
    buffer: CharBuffer<'src>,
    peeked: Option<Option<LocatedResult>>,
}

impl Iterator for Lexer<'_> {
    type Item = LocatedResult;

    /// The token is always a full match,
    /// never a substring that's more interesting than the whole match.
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();
        if result.is_none() {
            // Don't update `self.peeked` anymore,
            // if in the `Eof` state.
            return result
        }
        self.peeked = None;
        result
    }
}

impl FusedIterator for Lexer<'_> {}

impl<'src> Lexer<'src> {

    pub fn new(source: &'src str, standard_conforming_strings: bool) -> Self {
        Self {
            standard_conforming_strings,
            buffer: CharBuffer::new(source),
            peeked: None
        }
    }

    pub fn source(&self) -> &'src str {
        self.buffer.source()
    }

    /// Zero-length `range`.
    pub fn current_location(&self) -> Location {
        self.buffer.current_location()
    }

    pub fn peek(&mut self) -> Option<LocatedResult> {

        if let Some(result) = self.peeked.as_ref() {
            return result.clone()
        }

        let result = self.advance();
        self.peeked = Some(result.clone());
        result
    }

    fn advance(&mut self) -> Option<LocatedResult> {

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
            Err(err_code) => {
                let report = err_code.at(location);
                Some(Err(report))
            }
        }
    }

    fn lex_token(&mut self, concatenable_string: bool) -> Result {
        match self.buffer.consume_one().expect("eof should have already been filtered out") {
            '(' => Ok(Operator(OpenParenthesis)),
            ')' => Ok(Operator(CloseParenthesis)),
            ',' => Ok(Operator(Comma)),
            ';' => Ok(Operator(Semicolon)),
            '[' => Ok(Operator(OpenBracket)),
            ']' => Ok(Operator(CloseBracket)),
            '.' => {
                if self.buffer.consume_char('.') {
                    Ok(Operator(DotDot))
                }
                else if self.buffer.peek().is_some_and(is_decimal_digit) {
                    self.lex_dec_float()
                }
                else {
                    Ok(Operator(Dot))
                }
            }
            ':' => {
                if self.buffer.consume_char(':') {
                    Ok(Operator(Typecast))
                }
                else if self.buffer.consume_char('=') {
                    Ok(Operator(ColonEquals))
                }
                else {
                    Ok(Operator(Colon))
                }
            }
            '$' => match self.buffer.peek() {
                Some(c) if is_decimal_digit(c) => self.lex_param(),
                Some('$') => self.lex_dollar_string(), // empty delimiter
                Some(c) if is_ident_start(c) => self.lex_dollar_string(),
                _ => Err(UnexpectedChar { unknown: '$' }),
            }
            '\'' => {
                if self.standard_conforming_strings {
                    self.lex_quote_string(StringKind::Basic { concatenable: concatenable_string })
                }
                else {
                    self.lex_extended_string(concatenable_string)
                }
            }
            '"' => self.lex_quote_ident(Quoted),
            'b' | 'B' => {
                if self.buffer.consume_char('\'') {
                    return self.lex_bit_string(Binary)
                }
                self.lex_identifier()
            }
            'x' | 'X' => {
                if self.buffer.consume_char('\'') {
                    return self.lex_bit_string(Hex)
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

                if let Some('\'') = self.buffer.peek() {
                    return Ok(Kw(Nchar))
                }
                self.lex_identifier()
            }
            'u' | 'U' => {
                if self.buffer.consume_char('&') {
                    match self.buffer.peek() {
                        Some('\'') => { // u&'...'
                            if !self.standard_conforming_strings {
                                return Err(UnsafeUnicodeString)
                            }
                            self.buffer.consume_one();
                            self.lex_quote_string(StringKind::Unicode)
                        }
                        Some('"') => { // u&"..."
                            self.buffer.consume_one();
                            self.lex_quote_ident(IdentifierKind::Unicode)
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
                None => Ok(NumberLiteral(Decimal)),
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
                Err(UnexpectedChar { unknown })
            },
        }
    }

    #[inline] // Only called from a single place
    fn lex_operator(&mut self) -> Result {
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
            let c = self.buffer.consume_one()
                .expect("consuming inside a scope with peek");
            pg_op |= is_pg_op(c)
        }

        // SAFETY: Length is guaranteed to be at least 1.
        let mut op = self.buffer.slice(start_index);

        match op {
            "%"  => Ok(Operator(Percent)),
            "*"  => Ok(Operator(Mul)),
            "+"  => Ok(Operator(Plus)),
            "-"  => Ok(Operator(Minus)),
            "/"  => Ok(Operator(Div)),
            "<"  => Ok(Operator(Less)),
            "="  => Ok(Operator(Equals)),
            ">"  => Ok(Operator(Greater)),
            "^"  => Ok(Operator(Circumflex)),
            "=>" => Ok(Operator(EqualsGreater)),
            "<=" => Ok(Operator(LessEquals)),
            ">=" => Ok(Operator(GreaterEquals)),
            "!=" => Ok(Operator(NotEquals)),
            "<>" => Ok(Operator(NotEquals)),
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
                    self.buffer.seek(self.buffer.current_index() - num as u32);

                    let len = op.len() - num;
                    op = &op[..len];
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

    #[inline] // Only called from a single place
    fn lex_param(&mut self) -> Result {
        // $ has already been consumed, so no need to worry about it here

        let start_index = self.buffer.current_index();

        self.buffer.consume_while(is_decimal_digit);

        // check junk
        let consumed = self.buffer.consume_if(is_ident_start);
        if consumed.is_some() {
            return Err(TrailingJunkAfterParameter)
        }

        // SAFETY: They're all ASCII decimal digits
        let slice = self.buffer.slice(start_index).as_bytes();

        if slice.len() >= 10 && slice[0] > b'2' {
            // Careful with leading 0's.
            // Fail fast:
            //   The leading digit in i32::MAX is '2',
            //   so if the leading digit is above,
            //   then the string can't be safely parsed as an i32.
            return Err(ParameterNumberTooLarge)
        }

        // i32 is used to match original C-PG's expectation that it won't be > i32::MAX
        slice.iter()
            .map(|d| (d - b'0') as i32)
            .try_fold(0i32, |acc, n|
                acc.checked_mul(10)?.checked_add(n)
            )
            .map_or(
                Err(ParameterNumberTooLarge),
                |index| Ok(Param { index })
            )
    }

    #[inline] // Only called from a single place
    fn lex_dec_float(&mut self) -> Result {

        // \. {dec_digits} {dec_real}

        // The first char is '.' and it has already been consumed.
        // It's already known that the first dot is followed by a digit,
        // so there's no need to do any check here
        self.lex_dec_digits();
        self.lex_dec_real()?;
        Ok(NumberLiteral(Decimal))
    }

    fn lex_dec_integer(&mut self) -> Result {

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
                return Ok(NumberLiteral(Decimal))
            }
            // A trailing \. is valid, so this match is optional (empty match).
            self.lex_dec_digits();
        }

        self.lex_dec_real()?;

        Ok(NumberLiteral(Decimal))
    }

    fn lex_dec_real(&mut self) -> Result<()> {

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
            let underscore = self.buffer.consume_char('_');
            consumed = self.buffer.consume_while(is_decimal_digit);

            if consumed == 0 && underscore {
                self.buffer.push_back()
            }
        }

        true
    }

    fn lex_hex_integer(&mut self) -> Result {
        self.lex_prefixed_int(is_hex_digit, Hexadecimal)
    }

    fn lex_oct_integer(&mut self) -> Result {
        self.lex_prefixed_int(is_oct_digit, Octal)
    }

    fn lex_bin_integer(&mut self) -> Result {
        self.lex_prefixed_int(is_bin_digit, NumberRadix::Binary)
    }

    fn lex_prefixed_int(&mut self, is_digit: impl Fn(char) -> bool, radix: NumberRadix) -> Result {

        self.buffer.consume_one(); // ignore [xXoObB]

        let start_index = self.buffer.current_index();

        // /(_?{digit}+)*/
        let mut consumed = u32::MAX;
        while consumed > 0 {
            self.buffer.consume_char('_');
            consumed = self.buffer.consume_while(&is_digit);
        }

        // SAFETY: They're all ASCII chars ('_', or decimal & hex digits)
        let span = self.buffer.slice(start_index).as_bytes();

        if span.is_empty() || span.last().is_some_and(|c| *c == b'_') {
            return Err(InvalidInteger(radix))
        }

        if self.buffer.peek().is_some_and(is_ident_start) {
            return Err(TrailingJunkAfterNumericLiteral)
        }

        Ok(NumberLiteral(radix))
    }

    #[inline] // Only called from a single place
    fn lex_bit_string(&mut self, kind: BitStringKind) -> Result {

        // No content validation to simplify the lexer.

        loop {
            match self.buffer.consume_one() {
                None => {
                    let err = if kind == Hex {
                        UnterminatedHexString
                    }
                    else {
                        UnterminatedBitString
                    };
                    return Err(err)
                },
                Some('\'') => return Ok(BitStringLiteral(kind)),
                _ => {}
            }
        }
    }

    fn lex_quote_ident(&mut self, ident_kind: IdentifierKind) -> Result {

        let start_index = self.buffer.current_index();

        loop {
            let Some(c) = self.buffer.consume_one() else {
                return Err(UnterminatedQuotedIdentifier)
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
                Err(EmptyDelimitedIdentifier) // only consumed '"'
            }
            else {
                Ok(Identifier(ident_kind))
            }
        }
    }

    fn lex_quote_string(&mut self, kind: StringKind) -> Result {

        loop {
            let Some(c) = self.buffer.consume_one() else {
                return Err(UnterminatedQuotedString)
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

    fn lex_extended_string(&mut self, concatenable: bool) -> Result {

        // To keep the lexer simple, parsing escapes will be done at a later point.
        // This way the lexer doesn't need to work with Strings,
        // or have separate validation and parsing phases.

        loop {
            let Some(c) = self.buffer.consume_one() else {
                return Err(UnterminatedQuotedString)
            };

            if c == '\\' && self.buffer.consume_one().is_none() {
                return Err(UnterminatedQuotedString)
            }

            if c == '\'' {
                if let Some('\'') = self.buffer.peek() {
                    self.buffer.consume_one();
                }
                else {
                    return Ok(StringLiteral( Extended { concatenable } ))
                }
            }
        }
    }

    fn lex_identifier(&mut self) -> Result {

        // To prevent re-consuming it, {ident_start} was already consumed.
        let start_index = self.buffer.current_index() - 1;

        self.buffer.consume_while(is_ident_cont);

        let ident = self.buffer.slice(start_index);

        if let Some(kw) = Keyword::find(ident) {
            return Ok(Kw(kw))
        }

        Ok(Identifier(IdentifierKind::Basic))
    }

    fn lex_dollar_string(&mut self) -> Result {

        // The delimiter always contains '$' as the last char,
        // even if the delimiter is empty (i.e., '$$'),
        // so it's easier to match and consume.

        let Some(delim) = self.lex_dollar_delim() else {
            return Err(UnexpectedChar { unknown: '$' })
        };

        loop {
            if self.buffer.eof() {
                return Err(UnterminatedDollarQuotedString)
            }
            if self.buffer.consume_char('$') {
                if self.buffer.consume_string(delim) {
                    return Ok(StringLiteral(Dollar));
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
    fn skip_trivia(&mut self) -> lexer::LocatedResult<bool> {

        // Postgres:
        //   Returns Ok(true) if the whitespace contains \n and no block comments.
        //   https://github.com/postgres/postgres/blob/1d80d6b50e6401828fc445151375f9bde3f99ac6/src/backend/parser/scan.l#L244
        //
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

    fn skip_block_comment(&mut self) -> lexer::LocatedResult<bool> {

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
                let report = UnterminatedBlockComment.at(loc);
                return Err(report)
            }

            self.buffer.consume_one();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_kind::RawTokenKind;
    use crate::Keyword::{FromKw, Not, Select, StringKw};
    use core::ops::Range;
    use pg_elog::HasLocation;

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

        assert_tok(Operator(Dot), 0..1, 1, 1, lex.next());
        assert_tok(Operator(DotDot), 2..4, 1, 3, lex.next());
        assert_tok(Operator(OpenParenthesis), 5..6, 1, 6, lex.next());
        assert_tok(Operator(CloseParenthesis), 7..8, 1, 8, lex.next());
        assert_tok(Operator(Comma), 9..10, 1, 10, lex.next());
        assert_tok(Operator(Semicolon), 11..12, 1, 12, lex.next());
        assert_tok(Operator(OpenBracket), 13..14, 1, 14, lex.next());
        assert_tok(Operator(CloseBracket), 15..16, 1, 16, lex.next());
        assert_tok(Operator(Colon), 17..18, 1, 18, lex.next());
        assert_tok(Operator(Typecast), 19..21, 1, 20, lex.next());
        assert_tok(Operator(ColonEquals), 22..24, 1, 23, lex.next());
        assert_tok(Operator(Percent), 25..26, 1, 26, lex.next());
        assert_tok(Operator(Mul), 27..28, 1, 28, lex.next());
        assert_tok(Operator(Plus), 29..30, 1, 30, lex.next());
        assert_tok(Operator(Minus), 31..32, 1, 32, lex.next());
        assert_tok(Operator(Div), 33..34, 1, 34, lex.next());
        assert_tok(Operator(Less), 35..36, 1, 36, lex.next());
        assert_tok(Operator(Equals), 37..38, 1, 38, lex.next());
        assert_tok(Operator(Greater), 39..40, 1, 40, lex.next());
        assert_tok(Operator(Circumflex), 41..42, 1, 42, lex.next());
        assert_tok(Operator(EqualsGreater), 43..45, 1, 44, lex.next());
        assert_tok(Operator(LessEquals), 46..48, 1, 47, lex.next());
        assert_tok(Operator(GreaterEquals), 49..51, 1, 50, lex.next());
        assert_tok(Operator(NotEquals), 52..54, 1, 53, lex.next());
        assert_tok(Operator(NotEquals), 55..57, 1, 56, lex.next());
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
        assert_tok(Operator(Minus), 3..4, 1, 4, lex.next());
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

        assert_tok(NumberLiteral(Hexadecimal), 0..14, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_oct_number() {
        let source = "0o20155_53_7";
        let mut lex = Lexer::new(source, true);

        assert_tok(NumberLiteral(Octal), 0..12, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_bin_number() {
        let source = "0b1_001000_01001_01101";
        let mut lex = Lexer::new(source, true);

        assert_tok(NumberLiteral(NumberRadix::Binary), 0..22, 1, 1, lex.next());
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

        assert_tok(NumberLiteral(Decimal), 0..5, 1, 1, lex.next());
        assert_tok(NumberLiteral(Decimal), 6..13, 2, 1, lex.next());
        assert_tok(NumberLiteral(Decimal), 14..15, 3, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_integer_dot_dot() {
        let source = "184..";
        let mut lex = Lexer::new(source, true);

        assert_tok(NumberLiteral(Decimal), 0..3, 1, 1, lex.next());
        assert_tok(Operator(DotDot), 3..5, 1, 4, lex.next());
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

        assert_tok(NumberLiteral(Decimal), 0..16, 1, 1, lex.next());
        assert_tok(NumberLiteral(Decimal), 17..21, 2, 1, lex.next());
        assert_tok(NumberLiteral(Decimal), 22..25, 3, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string_with_empty_delim() {
        let source = "$$some string$$";
        let mut lex = Lexer::new(source, true);

        assert_tok(StringLiteral(Dollar), 0..15, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string() {
        let source = "$foo$bar baz$foo$";
        let mut lex = Lexer::new(source, true);

        assert_tok(StringLiteral(Dollar), 0..17, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string_with_dollars() {
        let source = "$foo$dolla $ dolla $$ bill$$foo$";
        let mut lex = Lexer::new(source, true);

        assert_tok(StringLiteral(Dollar), 0..32, 1, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_dollar_string_mismatch() {
        let source = "$not a string";
        let mut lex = Lexer::new(source, true);

        assert_err(UnexpectedChar { unknown: '$' }, 0..1, 1, 1, lex.next());
        assert_kw(Not, lex.next());
        assert_tok(Identifier(IdentifierKind::Basic), 5..6, 1, 6, lex.next());
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

        assert_tok(StringLiteral(StringKind::Basic { concatenable: false }), 0..2, 1, 1, lex.next());
        assert_tok(StringLiteral(StringKind::Basic { concatenable: true }), 3..17, 2, 1, lex.next());
        assert_tok(StringLiteral(StringKind::Basic { concatenable: false }), 18..23, 2, 16, lex.next());
        assert_tok(Kw(Nchar), 24..25, 3, 1, lex.next());
        assert_tok(StringLiteral(StringKind::Basic { concatenable: false }), 25..35, 3, 2, lex.next());
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

        assert_tok(StringLiteral(Extended { concatenable: false }), 0..12, 1, 1, lex.next());
        assert_tok(StringLiteral(Extended { concatenable: false }), 13..26, 2, 1, lex.next());
        assert_tok(Kw(Nchar), 27..28, 3, 1, lex.next());
        assert_tok(StringLiteral(Extended { concatenable: false }), 28..38, 3, 2, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_bit_string() {
        let source = "b'0_156e_wf' x'048_96a_f_d'"; // lexer doesn't validate chars
        let mut lex = Lexer::new(source, true);

        assert_tok(BitStringLiteral(Binary), 0..12, 1, 1, lex.next());
        assert_tok(BitStringLiteral(Hex), 13..27, 1, 14, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_unicode_string() {
        let source = "\
        u&''\n\
        U&'unicode\\'\
        ";
        let mut lex = Lexer::new(source, true);

        assert_tok(StringLiteral(StringKind::Unicode), 0..4, 1, 1, lex.next());
        assert_tok(StringLiteral(StringKind::Unicode), 5..17, 2, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_identifier() {
        let source = "bar xyz efg nun ube foo u&x";
        let mut lex = Lexer::new(source, true);
        assert_tok(Identifier(IdentifierKind::Basic), 0..3, 1, 1, lex.next());
        assert_tok(Identifier(IdentifierKind::Basic), 4..7, 1, 5, lex.next());
        assert_tok(Identifier(IdentifierKind::Basic), 8..11, 1, 9, lex.next());
        assert_tok(Identifier(IdentifierKind::Basic), 12..15, 1, 13, lex.next());
        assert_tok(Identifier(IdentifierKind::Basic), 16..19, 1, 17, lex.next());
        assert_tok(Identifier(IdentifierKind::Basic), 20..23, 1, 21, lex.next());
        assert_tok(Identifier(IdentifierKind::Basic), 24..25, 1, 25, lex.next());
        assert_tok(UserDefinedOperator, 25..26, 1, 26, lex.next());
        assert_tok(Identifier(IdentifierKind::Basic), 26..27, 1, 27, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_keyword() {
        let source = "SeLeCt FrOm";
        let mut lex = Lexer::new(source, true);
        assert_kw(Select, lex.next());
        assert_kw(FromKw, lex.next());
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
        assert_tok(Identifier(Quoted), 3..14, 2, 1, lex.next());
        assert_tok(Identifier(IdentifierKind::Unicode), 15..28, 3, 1, lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn test_peek() {
        let source = "two identifiers";
        let mut lex = Lexer::new(source, true);

        assert_tok(Identifier(IdentifierKind::Basic), 0..3, 1, 1, lex.peek());
        assert_tok(Identifier(IdentifierKind::Basic), 0..3, 1, 1, lex.next());

        assert_tok(Identifier(IdentifierKind::Basic), 4..15, 1, 5, lex.peek());
        assert_tok(Identifier(IdentifierKind::Basic), 4..15, 1, 5, lex.next());

        assert_eq!(None, lex.peek());
        assert_eq!(None, lex.next());
    }

    fn assert_tok(
        expected_kind: RawTokenKind,
        range: Range<u32>,
        line: u32,
        col: u32,
        actual: Option<LocatedResult>
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
        expected_err: lexer::Error,
        range: Range<u32>,
        line: u32,
        col: u32,
        actual: Option<LocatedResult>
    ) {
        let expected_loc = Location::new(range, line, col);

        assert_matches!(actual, Some(Err(err)) if err.source() == &expected_err && expected_loc.eq(err.location()));
    }

    fn assert_kw(expected: Keyword, actual: Option<LocatedResult>) {

        let (actual, _) = actual
            .expect("should have been Some(Ok(_))")
            .expect("should have been Ok((Keyword(_), _))");

        assert_matches!(actual, Kw(kw) if expected == kw)
    }
}

use self::Keyword::Nchar;
use crate::BitStringKind::Binary;
use crate::BitStringKind::Hex;
use crate::IdentifierKind::Quoted;
use crate::OperatorKind::Circumflex;
use crate::OperatorKind::CloseBracket;
use crate::OperatorKind::CloseParenthesis;
use crate::OperatorKind::Colon;
use crate::OperatorKind::ColonEquals;
use crate::OperatorKind::Comma;
use crate::OperatorKind::Div;
use crate::OperatorKind::Dot;
use crate::OperatorKind::DotDot;
use crate::OperatorKind::Equals;
use crate::OperatorKind::EqualsGreater;
use crate::OperatorKind::Greater;
use crate::OperatorKind::GreaterEquals;
use crate::OperatorKind::Less;
use crate::OperatorKind::LessEquals;
use crate::OperatorKind::Minus;
use crate::OperatorKind::Mul;
use crate::OperatorKind::NotEquals;
use crate::OperatorKind::OpenBracket;
use crate::OperatorKind::OpenParenthesis;
use crate::OperatorKind::Percent;
use crate::OperatorKind::Plus;
use crate::OperatorKind::Semicolon;
use crate::OperatorKind::Typecast;
use crate::RawTokenKind::BitStringLiteral;
use crate::RawTokenKind::Identifier;
use crate::RawTokenKind::Keyword as Kw;
use crate::RawTokenKind::NumberLiteral;
use crate::RawTokenKind::Operator;
use crate::RawTokenKind::Param;
use crate::RawTokenKind::StringLiteral;
use crate::RawTokenKind::UserDefinedOperator;
use crate::StringKind::Dollar;
use crate::StringKind::Extended;
use core::iter::FusedIterator;
use pg_basics::ascii::is_bin_digit;
use pg_basics::ascii::is_decimal_digit;
use pg_basics::ascii::is_dollar_quote_cont;
use pg_basics::ascii::is_hex_digit;
use pg_basics::ascii::is_ident_cont;
use pg_basics::ascii::is_ident_start;
use pg_basics::ascii::is_new_line;
use pg_basics::ascii::is_oct_digit;
use pg_basics::ascii::is_op;
use pg_basics::ascii::is_pg_op;
use pg_basics::ascii::is_whitespace;
use pg_basics::CharBuffer;
use pg_basics::Located;
use pg_basics::Location;
use pg_basics::NumberRadix;
use pg_basics::NumberRadix::Decimal;
use pg_basics::NumberRadix::Hexadecimal;
use pg_basics::NumberRadix::Octal;
use pg_basics::NAMEDATALEN;
use pg_elog::lexer;
use pg_elog::lexer::Error::EmptyDelimitedIdentifier;
use pg_elog::lexer::Error::InvalidInteger;
use pg_elog::lexer::Error::OperatorTooLong;
use pg_elog::lexer::Error::ParameterNumberTooLarge;
use pg_elog::lexer::Error::TrailingJunkAfterNumericLiteral;
use pg_elog::lexer::Error::TrailingJunkAfterParameter;
use pg_elog::lexer::Error::UnexpectedChar;
use pg_elog::lexer::Error::UnsafeUnicodeString;
use pg_elog::lexer::Error::UnterminatedBitString;
use pg_elog::lexer::Error::UnterminatedBlockComment;
use pg_elog::lexer::Error::UnterminatedDollarQuotedString;
use pg_elog::lexer::Error::UnterminatedHexString;
use pg_elog::lexer::Error::UnterminatedQuotedIdentifier;
use pg_elog::lexer::Error::UnterminatedQuotedString;
