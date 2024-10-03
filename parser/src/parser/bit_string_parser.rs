pub(super) struct BitStringParser<'p, 'src>(
    pub &'p mut Parser<'src>,
);

impl<'p, 'src> BitStringParser<'p, 'src> {

    pub fn parse(&mut self) -> OptResult<BitBox, BitStringError> {

        let Some((kind, loc)) = self.try_consume()? else { return Ok(None) };

        let slice = loc.slice(self.0.buffer.source());
        let slice = &slice[2..(slice.len() - 1)]; // strip delimiters
        let mut string = slice.to_owned();

        while let Ok(Some((suffix_kind, suffix_loc))) = self.try_consume_string() {
            let suffix_slice = suffix_loc.slice(self.0.buffer.source());
            let suffix_slice = strip_delimiters(suffix_kind, suffix_slice);
            string.push_str(suffix_slice);
        }

        self.decode(kind, &string)
    }

    fn try_consume(&mut self) -> OptResult<Located<BitStringKind>, BitStringError> {
        // let loc = self.0.buffer.current_location();
        // self.0.buffer.consume(|tok|
        //     tok.bit_string_kind()
        //         .map(|kind| (kind, loc.clone()))
        // )
        todo!()
    }

    fn try_consume_string(&mut self) -> OptResult<Located<StringKind>> {

        let loc = self.0.buffer.current_location();

        self.0.buffer.consume(|tok|
            tok.string_kind()
                .filter(|kind|
                    matches!(kind, BasicString { concatenable: true } | ExtendedString { concatenable: true })
                )
                .map(|kind| (kind, loc.clone()))
        )
    }

    fn decode(&mut self, kind: BitStringKind, slice: &str) -> OptResult<BitBox, BitStringError> {

        let result = BitStringDecoder::new(slice, kind == HexString)
            .decode();

        match result {
            Ok(result) => Ok(Some(result)),
            Err(err) => Err(Some(err))
        }
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

use crate::lexer::{
    BitStringKind,
    BitStringKind::HexString,
    StringKind,
    StringKind::*,
};
use crate::parser::{
    string_parser::strip_delimiters,
    token_buffer::TokenConsumer,
    OptResult,
    Parser,
};
use crate::string_decoders::{BitStringDecoder, BitStringError};
use bitvec::boxed::BitBox;
use postgres_basics::Located;
