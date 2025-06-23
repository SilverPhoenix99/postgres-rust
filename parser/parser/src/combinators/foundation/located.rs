macro_rules! located {
    ($stream:expr => $parser:expr) => {{
        use $crate::combinators::foundation::Combinator;

        let loc = $stream.current_location();
        $parser
            .parse($stream)
            .map(|ok| (ok, loc))
    }};

    ($parser:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            located!(stream => $parser)
        })
    };
}

pub(in crate::combinators) use located;
