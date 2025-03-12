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
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("unlisten *", OneOrAll::All)]
    #[test_case("unlisten test_name", OneOrAll::One("test_name".into()))]
    fn test_unlisten(source: &str, expected: OneOrAll<Str>) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), unlisten_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Unlisten;
use crate::lexer::OperatorKind::Mul;
use crate::parser::ast_node::OneOrAll;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
