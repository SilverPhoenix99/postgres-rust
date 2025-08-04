/// Alias: `opt_collate_clause`
pub(super) fn collate_clause(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        COLLATE any_name
    */

    let (_, collation) = seq!(Collate, any_name)
        .parse(stream)?;

    Ok(collation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("collate foo.bar" => Ok(vec!["foo".into(), "bar".into()]))]
    fn test_collate_clause(source: &str) -> scan::Result<QualifiedName> {
        test_parser!(source, collate_clause)
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Collate;
