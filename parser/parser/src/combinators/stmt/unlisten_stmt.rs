/// Alias: `UnlistenStmt`
pub(super) fn unlisten_stmt(stream: &mut TokenStream) -> scan::Result<OneOrAll<Str>> {

    /*
        UNLISTEN '*'
        UNLISTEN ColId
    */

    let (_, stmt) = seq!(=>
        Unlisten.parse(stream),
        choice!(parsed stream =>
            Mul.map(|_| OneOrAll::All),
            col_id.map(OneOrAll::One)
        )
    )?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("unlisten *", OneOrAll::All)]
    #[test_case("unlisten test_name", OneOrAll::One("test_name".into()))]
    fn test_unlisten(source: &str, expected: OneOrAll<Str>) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), unlisten_stmt(&mut stream));
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::OneOrAll;
use pg_basics::Str;
use pg_lexer::Keyword::Unlisten;
use pg_lexer::OperatorKind::Mul;
