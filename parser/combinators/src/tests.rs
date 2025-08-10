#[macro_export]
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
        let mut ctx = pg_parser_core::ParserContext::from(source);
        let parser = $parser;
        $crate::Combinator::parse(&parser, &mut ctx)
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
