#[macro_export]
macro_rules! located {
    ($parser:expr) => {
        $crate::combinators::core::parser(|ctx| {
            let loc = ctx.stream_mut().current_location();
            let p = $parser;
            let result = $crate::combinators::core::Combinator::parse(&p, ctx)?;
            Ok(pg_basics::Located(result, loc))
        })
    };
}
