macro_rules! located {
    ($stream:expr, $parser:expr) => {{
        let loc = $stream.current_location();
        $parser.map(|ok| (ok, loc))
    }};
}

pub(in crate::combinators) use located;
