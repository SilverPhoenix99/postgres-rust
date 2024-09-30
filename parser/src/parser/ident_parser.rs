pub(super) struct IdentifierParser<'p, 'src>(
    pub &'p mut Parser<'src>,
);

impl<'p, 'src> IdentifierParser<'p, 'src> {

    pub fn parse(&mut self) -> OptResult<String> {

        let loc = self.0.buffer.current_location();

        let result = self.0.buffer.consume(|tok|
            tok.identifier_kind()
                .map(|tok| (tok, loc.clone()))
        )?;

        let Some((kind, loc)) = result else {
            return Ok(None)
        };

        let slice = loc.slice(self.0.buffer.source());

        let ident = match kind {
            BasicIdentifier => {
                match std::str::from_utf8(slice) {
                    Ok(s) => Ok(s.to_string().to_lowercase()),
                    Err(err) => Err(err.into())
                }
            },
            QuotedIdentifier => {
                // Strip delimiters:
                let slice = &slice[1..slice.len() - 1];
                BasicStringDecoder::new(slice, true)
                    .decode()
                    .map_err(Utf8Error::into)
            },
            UnicodeIdentifier => {

                let escape = self.0.uescape()?;

                // Strip delimiters:
                let slice = &slice[3..slice.len() - 1];

                UnicodeStringDecoder::new(slice, true, escape)
                    .decode()
                    .map_err(ParserErrorKind::UnicodeString)
            },
        };

        let mut ident = ident.map_err(Some)?;

        if ident.len() > NAMEDATALEN {
            let len: usize = ident.chars()
                .take(NAMEDATALEN)
                .map(char::len_utf8)
                .sum();
            if len < ident.len() {
                ident.truncate(len);
            }
        }

        Ok(Some(ident))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Parser, ParserConfig};
    use postgres_basics::guc::BackslashQuote;

    #[test]
    fn test_basic_ident() {
        let mut parser = new_parser(b"sOmE_iDeNtIfIeR");
        let mut ident_parser = IdentifierParser(&mut parser);

        assert_eq!(Ok(Some("some_identifier".into())), ident_parser.parse());
    }

    #[test]
    fn test_quoted_ident() {
        let mut parser = new_parser(br#""quoted""#);
        let mut ident_parser = IdentifierParser(&mut parser);

        assert_eq!(Ok(Some("quoted".into())), ident_parser.parse());
    }

    #[test]
    fn test_unicode_ident() {
        let mut parser = new_parser(br#"u&"d\0061ta""#);
        let mut ident_parser = IdentifierParser(&mut parser);

        assert_eq!(Ok(Some("data".into())), ident_parser.parse());
    }

    #[test]
    fn test_unicode_ident_with_uescape() {
        let mut parser = new_parser(br#"u&"d!0061ta" UESCAPE '!'"#);
        let mut ident_parser = IdentifierParser(&mut parser);

        assert_eq!(Ok(Some("data".into())), ident_parser.parse());
    }

    fn new_parser(source: &[u8]) -> Parser<'_> {
        let config = ParserConfig::new(true, BackslashQuote::SafeEncoding);
        Parser::new(source, config)
    }
}

use crate::lexer::IdentifierKind::*;
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::{OptResult, Parser, ParserErrorKind};
use crate::string_decoders::{BasicStringDecoder, UnicodeStringDecoder};
use postgres_basics::NAMEDATALEN;
use std::str::Utf8Error;
