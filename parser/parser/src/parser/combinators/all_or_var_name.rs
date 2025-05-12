/// Alias: `generic_reset`
pub(super) fn all_or_var_name() -> impl Combinator<Output = OneOrAll<QualifiedName>> {

    /*
          ALL
        | var_name
    */

    match_first!(
        Keyword::All.map(|_| OneOrAll::All),
        var_name().map(OneOrAll::One)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("all", OneOrAll::All)]
    #[test_case("_ident", OneOrAll::One(vec!["_ident".into()]))]
    fn test_all_or_var_name(source: &str, expected: OneOrAll<QualifiedName>) {

        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = all_or_var_name().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::parser::ast_node::OneOrAll;
use crate::parser::ast_node::QualifiedName;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::var_name;
use postgres_parser_lexer::Keyword;
