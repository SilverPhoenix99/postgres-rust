/// Alias: `UnlistenStmt`
pub(super) fn unlisten_stmt(ctx: &mut ParserContext) -> scan::Result<OneOrAll<Str>> {

    /*
          UNLISTEN '*'
        | UNLISTEN ColId
    */

    let (_, stmt) = seq!(
        Unlisten,
        alt!(
            Mul.map(|_| OneOrAll::All),
            col_id.map(OneOrAll::One)
        )
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use pg_sink_ast::OneOrAll;
    use test_case::test_case;

    #[test_case("unlisten *" => Ok(OneOrAll::All))]
    #[test_case("unlisten test_name" => Ok(OneOrAll::One("test_name".into())))]
    fn test_unlisten(source: &str) -> scan::Result<OneOrAll<Str>> {
        test_parser!(source, unlisten_stmt)
    }
}

use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Unlisten;
use pg_lexer::OperatorKind::Mul;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
use pg_sink_ast::OneOrAll;
use pg_sink_combinators::col_id;
