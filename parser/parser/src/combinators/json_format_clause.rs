/// Alias: `json_format_clause_opt`
pub(super) fn json_format_clause(stream: &mut TokenStream) -> scan::Result<JsonFormat> {

    /*
        FORMAT JSON ( ENCODING ColId )?
    */

    let (_, _, encoding) = seq!(
        Format,
        Json,
        seq!(Encoding, located!(col_id)).optional()
    ).parse(stream)?;

    let encoding = encoding.map(|(_, encoding)| encoding);

    let format = JsonFormat::text();

    let Some((encoding, loc)) = encoding else {
        return Ok(format)
    };

    if encoding.is_ascii() {
        if encoding.eq_ignore_ascii_case("utf8") {
            return Ok(format.with_encoding(UTF8))
        }
        if encoding.eq_ignore_ascii_case("utf16") {
            return Ok(format.with_encoding(UTF16))
        }
        if encoding.eq_ignore_ascii_case("utf32") {
            return Ok(format.with_encoding(UTF32))
        }
    }

    let err = UnrecognizedJsonEncoding(encoding).at(loc);
    Err(err.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::JsonFormatKind::Text;
    use pg_elog::Error::Parser;
    use scan::Error::ScanErr;
    use test_case::test_case;

    #[test_case("format json" => Ok(JsonFormat::text()))]
    #[test_case("format json encoding UTF8" => Ok(
        JsonFormat::new(Some(Text), Some(UTF8))
    ))]
    #[test_case("format json encoding uTf16" => Ok(
        JsonFormat::new(Some(Text), Some(UTF16))
    ))]
    #[test_case("format json encoding utf32" => Ok(
        JsonFormat::new(Some(Text), Some(UTF32))
    ))]
    fn test_json_format_clause(source: &str) -> scan::Result<JsonFormat> {
        test_parser!(source, json_format_clause)
    }

    #[test]
    fn test_err() {
        let actual = test_parser!("format json encoding en_US", json_format_clause);

        assert_matches!(actual, Err(ScanErr(_)));
        let Err(ScanErr(err)) = actual else { unreachable!() };

        assert_eq!(&Parser(UnrecognizedJsonEncoding("en_us".into())), err.source());
    }

}

use crate::combinators::col_id;
use crate::combinators::foundation::located;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::stream::TokenStream;
use pg_ast::JsonEncoding::UTF16;
use pg_ast::JsonEncoding::UTF32;
use pg_ast::JsonEncoding::UTF8;
use pg_ast::JsonFormat;
use pg_elog::parser::Error::UnrecognizedJsonEncoding;
use pg_lexer::Keyword::Encoding;
use pg_lexer::Keyword::Format;
use pg_lexer::Keyword::Json;
use pg_parser_core::scan;
