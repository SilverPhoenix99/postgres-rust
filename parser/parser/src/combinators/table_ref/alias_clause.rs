/// Alias: `opt_alias_clause`
pub(super) fn alias_clause(stream: &mut TokenStream) -> scan::Result<Alias> {

    /*
        ( AS )? ColId ( '(' name_list ')' )?
    */

    let (_, name, columns) = seq!(
        As.optional(),
        col_id,
        paren!(name_list).optional()
    ).parse(stream)?;

    let mut alias = Alias::new(name);
    alias.set_columns(columns);

    Ok(alias)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("foo" => Ok(Alias::new("foo")))]
    #[test_case("as bar" => Ok(Alias::new("bar")))]
    #[test_case("baz(lorem)" => Ok(
        Alias::new("baz")
            .with_columns(vec!["lorem".into()])
    ))]
    #[test_case("as qux(ipsum)" => Ok(
        Alias::new("qux")
            .with_columns(vec!["ipsum".into()])
    ))]
    fn test_alias_clause(source: &str) -> scan::Result<Alias> {
        test_parser!(source, alias_clause)
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::name_list;
use crate::stream::TokenStream;
use pg_ast::Alias;
use pg_lexer::Keyword::As;
use pg_parser_core::scan;
