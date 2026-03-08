#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod combinators;

pg_basics::reexport! { pub
    parser,
}

pg_basics::reexport! { pub(crate)
    context,
}

fn no_match<T>(ctx: &mut ParserContext) -> pg_parser_core::scan::Result<T> {
    Err(pg_parser_core::scan::Error::NoMatch(ctx.stream_mut().current_location()))
}
