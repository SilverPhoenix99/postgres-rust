macro_rules! located {
    ($parser:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            let loc = pg_parser_core::stream::TokenStream::current_location(stream);
            let p = $parser;
            let result = pg_combinators::Combinator::parse(&p, stream)?;
            Ok((result, loc))
        })
    };
}

pub(in crate::combinators) use located;
