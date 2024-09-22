use crate::lexer::IdentifierKind::*;
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::{OptResult, Parser, ParserError};
use crate::string_decoders::{BasicStringDecoder, UnicodeStringDecoder};
use postgres_basics::NAMEDATALEN;
use std::str::Utf8Error;

#[repr(transparent)]
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

        let (kind, loc) = match result {
            Some((kind, loc)) => (kind, loc),
            None => return Ok(None),
        };

        let slice = loc.slice(self.0.source);

        let ident = match kind {
            BasicIdentifier => {
                std::str::from_utf8(slice)
                    .map(String::from)
                    .map_err(Utf8Error::into)
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

                UnicodeStringDecoder::new(slice, true, escape)
                    .decode()
                    .map_err(ParserError::UnicodeString)
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
