/// Alias: `UnlistenStmt`
pub(super) fn unlisten_stmt() -> impl Combinator<Output = OneOrAll<Str>> {

    /*
        UNLISTEN '*'
        UNLISTEN ColId
    */

    Unlisten
        .and_right(match_first!{
            Mul.map(|_| OneOrAll::All),
            col_id().map(OneOrAll::One)
        })
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
        assert_eq!(Ok(expected), unlisten_stmt().parse(&mut stream));
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::OneOrAll;
use pg_basics::Str;
use pg_lexer::Keyword::Unlisten;
use pg_lexer::OperatorKind::Mul;
