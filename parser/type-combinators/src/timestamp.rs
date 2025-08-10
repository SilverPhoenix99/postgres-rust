/// Inlined: `ConstDatetime`
pub(super) fn timestamp(ctx: &mut ParserContext) -> scan::Result<TypeName> {

    /*
        TIMESTAMP ( '(' ICONST ')' )? ( with_timezone )?
    */

    let (_, precision, with_tz) = seq!(
        Kw::Timestamp,
        precision.optional(),
        with_timezone.optional()
            .map(Option::unwrap_or_default)
    ).parse(ctx)?;

    let typ = if with_tz {
        TimestampTz { precision }
    }
    else {
        Timestamp { precision }
    };

    Ok(typ)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("timestamp"                      => Ok(Timestamp { precision: None }))]
    #[test_case("timestamp(5)"                   => Ok(Timestamp { precision: Some(5) }))]
    #[test_case("timestamp without time zone"    => Ok(Timestamp { precision: None }))]
    #[test_case("timestamp(7) without time zone" => Ok(Timestamp { precision: Some(7) }))]
    #[test_case("timestamp with time zone"       => Ok(TimestampTz { precision: None }))]
    #[test_case("timestamp(9) with time zone"    => Ok(TimestampTz { precision: Some(9) }))]
    fn test_timestamp(source: &str) -> scan::Result<TypeName> {
        test_parser!(source, timestamp)
    }
}

use crate::with_timezone;
use pg_ast::TypeName;
use pg_ast::TypeName::Timestamp;
use pg_ast::TypeName::TimestampTz;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
use pg_sink_combinators::precision;
