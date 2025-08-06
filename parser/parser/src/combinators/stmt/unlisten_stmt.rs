/// Alias: `UnlistenStmt`
pub(super) fn unlisten_stmt(stream: &mut TokenStream) -> scan::Result<OneOrAll<Str>> {

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
    ).parse(stream)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("unlisten *" => Ok(OneOrAll::All))]
    #[test_case("unlisten test_name" => Ok(OneOrAll::One("test_name".into())))]
    fn test_unlisten(source: &str) -> scan::Result<OneOrAll<Str>> {
        test_parser!(source, unlisten_stmt)
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use pg_ast::OneOrAll;
use pg_basics::Str;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Unlisten;
use pg_lexer::OperatorKind::Mul;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
