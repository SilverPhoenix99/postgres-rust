#![cfg(test)]

macro_rules! test_parser {

    (internal =>
        $source:expr,
        $parser:expr,
        $expected:expr
    ) => {{
        let actual = test_parser!($source, $parser);
        let expected = $expected;
        assert_eq!(expected, actual);
    }};

    ($source:expr, $parser:expr) => {{
        let source = $source;
        let mut stream = $crate::stream::TokenStream::from(source);
        let parser = $parser;
        $crate::combinators::foundation::Combinator::parse(&parser, &mut stream)
    }};

    (
        source = $source:expr,
        parser = $parser:expr
    ) => {
        test_parser!($source, $parser)
    };

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

pub(crate) use test_parser;
