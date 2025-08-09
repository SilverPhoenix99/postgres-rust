#[macro_export]
macro_rules! located {
    ($parser:expr) => {
        $crate::parser(|stream| {
            let loc = pg_parser_core::stream::TokenStream::current_location(stream);
            let p = $parser;
            let result = $crate::Combinator::parse(&p, stream)?;
            Ok(pg_basics::Located(result, loc))
        })
    };
}
