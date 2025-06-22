macro_rules! located {
    ($parser:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            use $crate::combinators::foundation::Combinator;

            let loc = stream.current_location();
            $parser
                .parse(stream)
                .map(|ok| (ok, loc))
        })
    };
}

pub(in crate::combinators) use located;
