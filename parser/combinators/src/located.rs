#[macro_export]
macro_rules! located {
    ($parser:expr) => {
        $crate::parser(|ctx| {
            let loc = ctx.stream_mut().current_location();
            let p = $parser;
            let result = $crate::Combinator::parse(&p, ctx)?;
            Ok(pg_basics::Located(result, loc))
        })
    };
}
