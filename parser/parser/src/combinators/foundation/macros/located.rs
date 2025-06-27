macro_rules! located {
    ($stream:ident => $parser:expr) => {{
        let loc = $crate::stream::TokenStream::current_location($stream);
        let p = $parser;
        let result = $crate::combinators::foundation::Combinator::parse(&p, $stream);
        result.map(|ok| (ok, loc))
    }};

    ($parser:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            $crate::combinators::foundation::located!(stream =>
                $parser
            )
        })
    };
}

pub(in crate::combinators) use located;
