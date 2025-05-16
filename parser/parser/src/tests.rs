#![cfg(test)]

pub(crate) static DEFAULT_CONFIG: ParserConfig = ParserConfig::new(true, SafeEncoding);

macro_rules! test_parser {
    (
        source = $source:expr,
        parser = $parser:expr,
        expected = $expected:expr
    ) => {{
        test_parser!($source, $parser, $expected)
    }};

    (
        $source:expr,
        $parser:expr,
        $expected:expr
    ) => {{
        use crate::tests::DEFAULT_CONFIG;
        use crate::stream::TokenStream;

        let mut stream = TokenStream::new($source, DEFAULT_CONFIG);
        let actual = $parser.parse(&mut stream);

        let expected = $expected;

        assert_eq!(Ok(expected), actual);
    }};
}

pub(crate) use test_parser;

use crate::ParserConfig;
use pg_basics::guc::BackslashQuote::SafeEncoding;
