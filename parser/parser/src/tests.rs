#![cfg(test)]

macro_rules! test_parser {

    (
        source = $source:expr,
        parser = $parser:expr
    ) => {
        test_parser!($source, $parser)
    };

    (
        source = $source:expr,
        parser = $parser:expr,
        expected = $expected:expr
    ) => {
        test_parser!($source, $parser, $expected)
    };

    ($source:expr, $parser:expr) => {{
        let source = $source;
        let mut stream = pg_parser_core::stream::TokenStream::from(source);
        let parser = $parser;
        $crate::combinators::foundation::Combinator::parse(&parser, &mut stream)
    }};

    (
        $source:expr,
        $parser:expr,
        $expected:expr
    ) => {{
        let actual = test_parser!($source, $parser);
        let expected = $expected;
        assert_eq!(Ok(expected.into()), actual);
    }};
}

pub(crate) use test_parser;
