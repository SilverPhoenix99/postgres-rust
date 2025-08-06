/// Alias: `generic_reset`
pub(in crate::combinators) fn all_or_var_name(stream: &mut TokenStream) -> scan::Result<OneOrAll<QualifiedName>> {

    /*
          ALL
        | var_name
    */

    alt!(
        Keyword::All.map(|_| OneOrAll::All),
        var_name.map(OneOrAll::One)
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::stream;
    use test_case::test_case;

    #[test_case("all", OneOrAll::All)]
    #[test_case("_ident", OneOrAll::One(vec!["_ident".into()]))]
    fn test_all_or_var_name(source: &str, expected: OneOrAll<QualifiedName>) {

        let mut stream = stream(source);
        let actual = all_or_var_name(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::Combinator;
use crate::combinators::var_name;
use crate::stream::TokenStream;
use pg_ast::OneOrAll;
use pg_basics::QualifiedName;
use pg_lexer::Keyword;
use pg_parser_core::scan;
