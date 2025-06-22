#![cfg(test)]

pub(crate) static DEFAULT_CONFIG: ParserConfig = ParserConfig::new(true, SafeEncoding);

macro_rules! test_parser {
    (
        source = $source:expr,
        parser = $parser:expr,
        expected = $expected:expr
    ) => {
        test_parser!($source, $parser, $expected)
    };

    (
        $source:expr,
        $parser:expr,
        $expected:expr
    ) => {{
        #[allow(unused_imports)]
        use $crate::combinators::foundation::{ClosureHelpers, Combinator, CombinatorHelpers};
        use $crate::tests::stream;

        let mut stream = stream($source);
        let actual = $parser.parse(&mut stream);

        let expected = $expected.into();

        assert_eq!(Ok(expected), actual);
    }};
}

pub(crate) fn stream(source: &str) -> TokenStream {
    TokenStream::new(source, DEFAULT_CONFIG)
}

pub(crate) use test_parser;

use crate::ParserConfig;
use pg_basics::guc::BackslashQuote::SafeEncoding;
use crate::stream::TokenStream;
