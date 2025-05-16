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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("all", OneOrAll::All)]
    #[test_case("_ident", OneOrAll::One(vec!["_ident".into()]))]
    fn test_all_or_var_name(source: &str, expected: OneOrAll<QualifiedName>) {

        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = all_or_var_name().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::var_name;
use pg_ast::OneOrAll;
use pg_basics::QualifiedName;
use pg_lexer::Keyword;
