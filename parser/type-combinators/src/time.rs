/// Inlined: `ConstDatetime`
pub(super) fn time(ctx: &mut ParserContext) -> scan::Result<TypeName> {

    /*
        TIME ( '(' ICONST ')' )? ( with_timezone )?
    */

    let (_, precision, with_tz) = seq!(
        Kw::Time,
        precision.optional(),
        with_timezone.optional()
            .map(Option::unwrap_or_default)
    ).parse(ctx)?;

    let typ = if with_tz {
        TimeTz { precision }
    }
    else {
        Time { precision }
    };

    Ok(typ)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("time"                      => Ok(Time { precision: None }))]
    #[test_case("time(5)"                   => Ok(Time { precision: Some(5) }))]
    #[test_case("time without time zone"    => Ok(Time { precision: None }))]
    #[test_case("time(7) without time zone" => Ok(Time { precision: Some(7) }))]
    #[test_case("time with time zone"       => Ok(TimeTz { precision: None }))]
    #[test_case("time(9) with time zone"    => Ok(TimeTz { precision: Some(9) }))]
    fn test_time(source: &str) -> scan::Result<TypeName> {
        test_parser!(source, time)
    }
}

use crate::with_timezone;
use pg_ast::TypeName;
use pg_ast::TypeName::Time;
use pg_ast::TypeName::TimeTz;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
use pg_sink_combinators::precision;
