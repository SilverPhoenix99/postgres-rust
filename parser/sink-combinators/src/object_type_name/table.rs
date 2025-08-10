pub fn table(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

    /*
        TABLE any_name
    */

    let (_, name) = seq!(Table, any_name)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_table() {
        test_parser!(
            source = "table foo",
            parser = table,
            expected = vec!["foo".into()]
        )
    }
}

use crate::any_name;
use pg_basics::QualifiedName;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Table;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
