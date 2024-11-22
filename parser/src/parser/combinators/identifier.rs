
/// Aliases:
/// * `IDENT`
/// * `UIDENT`
pub(in crate::parser) fn identifier() -> IdentifierCombi {
    IdentifierCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct IdentifierCombi;

impl Combinator for IdentifierCombi {
    type Output = Box<str>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {

        let (kind, slice, loc) = stream.consume_with_slice(|(tok, slice, loc)|
            tok.identifier()
                .map(|kind| (kind, slice, loc))
        )?;

        let mut ident = match kind {
            Basic => slice.to_lowercase(),
            Quoted => {
                // Strip delimiters:
                let slice = &slice[1..slice.len() - 1];
                let ident = BasicStringDecoder::new(slice, true).decode();
                ident.into_string()
            }

            IdentifierKind::Unicode => {

                let escape = uescape().parse(stream)?;

                // Strip delimiters:
                let slice = &slice[3..slice.len() - 1];

                UnicodeStringDecoder::new(slice, true, escape)
                    .decode()
                    .map(str::into_string)
                    .map_err(|err|
                        ParserError::new(UnicodeString(err), loc)
                    )?
            }
        };

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
    use test_case::test_case;

    #[test_case("sOmE_iDeNtIfIeR", "some_identifier")]
    #[test_case(r#""quoted""#, "quoted")]
    #[test_case(r#"u&"d\0061ta""#, "data")]
    #[test_case(r#"u&"d!0061ta" UESCAPE '!'"#, "data")]
    fn test_identifier(source: &str, expected: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let parser = identifier();
        let actual = parser.parse(&mut stream);
        assert_eq!(expected, actual.unwrap().as_ref())
    }
}

use crate::{
    lexer::IdentifierKind::{self, Basic, Quoted},
    parser::{
        combinators::{
            string::uescape,
            Combinator
        },
        result::ScanResult,
        token_stream::{SlicedTokenConsumer, TokenStream},
        ParserError,
        ParserErrorKind::UnicodeString
    },
    string_decoders::{BasicStringDecoder, UnicodeStringDecoder}
};
use postgres_basics::NAMEDATALEN;
