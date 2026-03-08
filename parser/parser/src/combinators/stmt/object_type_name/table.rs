pub(in crate::combinators::stmt) fn table(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

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
    use crate::test_parser;

    #[test]
    fn test_table() {
        test_parser!(
            source = "table foo",
            parser = table,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Table;
use pg_parser_core::scan;
