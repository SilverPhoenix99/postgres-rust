#![cfg(test)]

pub(crate) static DEFAULT_CONFIG: ParserConfig = ParserConfig::new(true, SafeEncoding);

macro_rules! test_parser {

    (internal =>
        $source:expr,
        $parser:expr,
        $expected:expr
    ) => {{
        #[allow(unused_imports)]
        use $crate::combinators::foundation::Combinator;
        use $crate::tests::stream;

        let mut stream = stream($source);
        let actual = $parser.parse(&mut stream);

        let expected = $expected;

        assert_eq!(expected, actual);
    }};

    (
        source = $source:expr,
        parser = $parser:expr,
        expected = Err($expected:expr)
    ) => {
        test_parser!(internal => $source, $parser, Err($expected.into()))
    };

    (
        source = $source:expr,
        parser = $parser:expr,
        expected = Ok($expected:expr)
    ) => {
        test_parser!(internal => $source, $parser, Ok($expected.into()))
    };

    (
        source = $source:expr,
        parser = $parser:expr,
        expected = $expected:expr
    ) => {
        test_parser!(internal => $source, $parser, Ok($expected.into()))
    };

    (
        $source:expr,
        $parser:expr,
        Err($expected:expr)
    ) => {
        test_parser!(internal => $source, $parser, Err($expected.into()))
    };

    (
        $source:expr,
        $parser:expr,
        Ok($expected:expr)
    ) => {
        test_parser!(internal => $source, $parser, Ok($expected.into()))
    };

    (
        $source:expr,
        $parser:expr,
        $expected:expr
    ) => {
        test_parser!(internal => $source, $parser, Ok($expected.into()))
    };
}

pub(crate) fn stream(source: &str) -> TokenStream<'_> {
    TokenStream::new(source, DEFAULT_CONFIG)
}

pub(crate) use test_parser;

use crate::ParserConfig;
use pg_basics::guc::BackslashQuote::SafeEncoding;
use crate::stream::TokenStream;
