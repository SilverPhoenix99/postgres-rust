pub(super) struct BitStringParser<'p, 'src>(
    pub &'p mut Parser<'src>,
);

impl<'p, 'src> BitStringParser<'p, 'src> {

    pub fn parse(&mut self) -> ScanResult<BitBox> {

        let (kind, slice) = self.try_consume()?;

        let slice = &slice[2..(slice.len() - 1)]; // strip delimiters
        let mut string = slice.to_owned();

        while let Ok((suffix_kind, suffix_slice)) = self.try_consume_string() {
            let suffix_slice = strip_delimiters(suffix_kind, suffix_slice);
            string.push_str(suffix_slice);
        }

        let result = BitStringDecoder::new(slice, kind == Hex).decode();
        todo!("map error")
    }

    fn try_consume(&mut self) -> ScanResult<(BitStringKind, &'src str)> {
        // let loc = self.0.buffer.current_location();
        // self.0.buffer.consume(|tok|
        //     tok.bit_string_kind()
        // )
        todo!()
    }

    fn try_consume_string(&mut self) -> ScanResult<(StringKind, &'src str)> {

        self.0.buffer.consume_with_slice(|(tok, slice, _)|
            tok.string()
                .filter(|kind|
                    matches!(kind, Basic { concatenable: true } | Extended { concatenable: true })
                )
                .map(|kind| (kind, slice))
        )
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ParserConfig;
    use bitvec::bitvec;
    use bitvec::prelude::Lsb0;
    use postgres_basics::guc::BackslashQuote;

    #[test]
    fn test_parse_binary_string() {
        let mut parser = new_parser(
            "b'0110'\n\
            '1001'"
        );
        let mut bit_string_parser = BitStringParser(&mut parser);

        let expected = bitvec![0, 1, 1, 0, 1, 0, 0, 1].into_boxed_bitslice();

        let result = bit_string_parser.parse();
        assert_eq!(Ok(Some(expected)), result);
    }

    #[test]
    fn test_parse_hex_string() {
        let mut parser = new_parser(
            "x'1f'\n\
            'a9'"
        );
        let mut bit_string_parser = BitStringParser(&mut parser);

        let expected = bitvec![
            0, 0, 0, 1,
            1, 1, 1, 1,
            1, 0, 1, 0,
            1, 0, 0, 1
        ].into_boxed_bitslice();

        let result = bit_string_parser.parse();
        assert_eq!(Ok(Some(expected)), result);
    }

    fn new_parser(source: &str) -> Parser<'_> {
        let config = ParserConfig::new(true, BackslashQuote::SafeEncoding);
        Parser::new(source, config)
    }
}
*/

use crate::{
    lexer::{BitStringKind, BitStringKind::Hex, StringKind, StringKind::*},
    parser::{
        result::ScanResult,
        string_parser::strip_delimiters,
        token_stream::SlicedTokenConsumer,
        Parser
    },
    string_decoders::BitStringDecoder,
};
use bitvec::boxed::BitBox;
