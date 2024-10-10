pub(super) struct IdentifierParser<'p, 'src>(
    pub &'p mut Parser<'src>,
);

impl<'p, 'src> IdentifierParser<'p, 'src> {

    pub fn parse(&mut self) -> Result<String, ScanErrorKind> {

        let loc = self.0.buffer.current_location();
        let kind = self.0.buffer.consume(TokenKind::identifier_kind)?;
        let slice = loc.slice(self.0.buffer.source());

        let ident = match kind {
            BasicIdentifier => Ok(slice.to_string().to_lowercase()),
            QuotedIdentifier => {
                // Strip delimiters:
                let slice = &slice[1..slice.len() - 1];
                let ident = BasicStringDecoder::new(slice, true).decode();
                Ok(ident)
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

        let mut ident = ident.map_err(ScanErrorKind::from)?;

        if ident.len() > NAMEDATALEN {
            let len: usize = ident.chars()
                .take(NAMEDATALEN)
                .map(char::len_utf8)
                .sum();
            if len < ident.len() {
                ident.truncate(len);
            }
        }

        Ok(ident)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Parser, ParserConfig};
    use postgres_basics::guc::BackslashQuote;

    #[test]
    fn test_basic_ident() {
        let mut parser = new_parser("sOmE_iDeNtIfIeR");
        let mut ident_parser = IdentifierParser(&mut parser);

        assert_eq!(Ok("some_identifier".into()), ident_parser.parse());
    }

    #[test]
    fn test_quoted_ident() {
        let mut parser = new_parser(r#""quoted""#);
        let mut ident_parser = IdentifierParser(&mut parser);

        assert_eq!(Ok("quoted".into()), ident_parser.parse());
    }

    #[test]
    fn test_unicode_ident() {
        let mut parser = new_parser(r#"u&"d\0061ta""#);
        let mut ident_parser = IdentifierParser(&mut parser);

        assert_eq!(Ok("data".into()), ident_parser.parse());
    }

    #[test]
    fn test_unicode_ident_with_uescape() {
        let mut parser = new_parser(r#"u&"d!0061ta" UESCAPE '!'"#);
        let mut ident_parser = IdentifierParser(&mut parser);

        assert_eq!(Ok("data".into()), ident_parser.parse());
    }

    fn new_parser(source: &str) -> Parser<'_> {
        let config = ParserConfig::new(true, BackslashQuote::SafeEncoding);
        Parser::new(source, config)
    }
}

use crate::lexer::IdentifierKind::*;
use crate::lexer::TokenKind;
use crate::parser::result::ScanErrorKind;
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::{Parser, ParserErrorKind};
use crate::string_decoders::{BasicStringDecoder, UnicodeStringDecoder};
use postgres_basics::NAMEDATALEN;
