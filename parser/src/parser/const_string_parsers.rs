impl Parser<'_> {

    /// Aliases:
    /// * `SCONST`
    /// * `USCONST`
    /// * `file_name`
    #[inline(always)]
    pub(in crate::parser) fn string(&mut self) -> ScanResult<String> {
        StringParser(self).parse()
    }

    pub(in crate::parser) fn bit_string(&mut self) -> ScanResult<(BitStringKind, String)> {

        let slice = self.buffer.slice();
        let kind = self.buffer.consume(|tok| tok.bit_string_kind())?;
        let slice = slice.expect("slice is valid due to previous consume");

        // strip delimiters
        let slice = &slice[2..(slice.len() - 1)];

        Ok((kind, slice.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test]
    fn test_string() {
        let source = "'test string'";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        assert_eq!("test string", parser.string().unwrap());
    }

    #[test_case("b'0101'", BitStringKind::Binary, "0101")]
    #[test_case("x'19af'", BitStringKind::Hex, "19af")]
    fn test_bit_string(source: &str, expected_kind: BitStringKind, expected_value: &str) {
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.bit_string();
        assert_eq!(Ok((expected_kind, expected_value.to_string())), actual);
    }
}

use crate::{
    lexer::BitStringKind,
    parser::{
        result::ScanResult,
        string_parser::StringParser,
        token_buffer::TokenConsumer,
        Parser
    }
};
