macro_rules! located {
    ($parser:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            #[allow(unused_imports)]
            use $crate::combinators::foundation::{ClosureHelpers, CombinatorHelpers};

            let loc = stream.current_location();
            $parser
                .parse(stream)
                .map(|ok| (ok, loc))
        })
    };
}

pub(in crate::combinators) use located;
