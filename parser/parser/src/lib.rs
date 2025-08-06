extern crate alloc;
#[cfg(test)]
#[macro_use]
extern crate assert_matches;
extern crate core;

mod combinators;

pg_basics::reexport! { pub
    parser,
}

fn no_match<T>(stream: &mut pg_parser_core::stream::TokenStream) -> pg_parser_core::scan::Result<T> {
    Err(pg_parser_core::scan::Error::NoMatch(stream.current_location()))
}
