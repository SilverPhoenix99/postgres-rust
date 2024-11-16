pub(super) struct IdentifierParser<'p, 'src>(
    pub &'p mut TokenStream<'src>,
);

impl<'p, 'src> IdentifierParser<'p, 'src> {

    pub fn parse(&mut self) -> ScanResult<Box<str>> {

        let (kind, slice, loc) = self.0.consume_with_slice(|(tok, slice, loc)|
            tok.identifier()
                .map(|kind| (kind, slice, loc))
        )?;

        let ident = match kind {
            Basic => Ok(slice.to_lowercase()),
            Quoted => {
                // Strip delimiters:
                let slice = &slice[1..slice.len() - 1];
                let ident = BasicStringDecoder::new(slice, true).decode();
                Ok(ident.into_string())
            }
            Unicode => {

                let escape = self.0.uescape()?;

                // Strip delimiters:
                let slice = &slice[3..slice.len() - 1];

                UnicodeStringDecoder::new(slice, true, escape)
                    .decode()
                    .map(str::into_string)
                    .map_err(|err|
                        ParserError::new(UnicodeString(err), fn_info!(), loc)
                    )
            },
        };

        let mut ident = ident?;

        if ident.len() > NAMEDATALEN {
            let len: usize = ident.chars()
                .take(NAMEDATALEN)
                .map(char::len_utf8)
                .sum();
            if len < ident.len() {
                ident.truncate(len);
            }
        }

        Ok(ident.into_boxed_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_basic_ident() {
        let mut token_stream = TokenStream::new("sOmE_iDeNtIfIeR", DEFAULT_CONFIG);
        let mut ident_parser = IdentifierParser(&mut token_stream);

        assert_eq!(Ok("some_identifier".into()), ident_parser.parse());
    }

    #[test]
    fn test_quoted_ident() {
        let mut token_stream = TokenStream::new(r#""quoted""#, DEFAULT_CONFIG);
        let mut ident_parser = IdentifierParser(&mut token_stream);

        assert_eq!(Ok("quoted".into()), ident_parser.parse());
    }

    #[test]
    fn test_unicode_ident() {
        let mut token_stream = TokenStream::new(r#"u&"d\0061ta""#, DEFAULT_CONFIG);
        let mut ident_parser = IdentifierParser(&mut token_stream);

        assert_eq!(Ok("data".into()), ident_parser.parse());
    }

    #[test]
    fn test_unicode_ident_with_uescape() {
        let mut token_stream = TokenStream::new(r#"u&"d!0061ta" UESCAPE '!'"#, DEFAULT_CONFIG);
        let mut ident_parser = IdentifierParser(&mut token_stream);

        assert_eq!(Ok("data".into()), ident_parser.parse());
    }
}

use crate::{
    lexer::IdentifierKind::*,
    parser::{
        result::ScanResult,
        token_stream::{SlicedTokenConsumer, TokenStream},
        ParserError,
        ParserErrorKind::UnicodeString
    },
    string_decoders::{BasicStringDecoder, UnicodeStringDecoder}
};
use postgres_basics::{fn_info, NAMEDATALEN};
