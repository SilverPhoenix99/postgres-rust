#![cfg(test)]

pub(crate) static DEFAULT_CONFIG: ParserConfig = ParserConfig::new(true, SafeEncoding);

macro_rules! test_parser {
    (
        v2,
        source = $source:expr,
        parser = $parser:expr,
        expected = $expected:expr
    ) => {{
        test_parser!(v2, $source, $parser, $expected)
    }};

    (
        v2,
        $source:expr,
        $parser:expr,
        $expected:expr
    ) => {{
        use $crate::tests::stream;

        let mut stream = stream($source);
        let actual = $parser(&mut stream);

        let expected = $expected;

        assert_eq!(Ok(expected), actual);
    }};
    
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
        use $crate::tests::stream;

        let mut stream = stream($source);
        let actual = $parser.parse(&mut stream);

        let expected = $expected;

        assert_eq!(Ok(expected), actual);
    }};
}

pub(crate) fn stream(source: &str) -> TokenStream {
    TokenStream::new(source, DEFAULT_CONFIG)
}

pub(crate) use test_parser;

use crate::stream::TokenStream;
use crate::ParserConfig;
use pg_basics::guc::BackslashQuote::SafeEncoding;
