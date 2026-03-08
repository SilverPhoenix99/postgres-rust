pub(in crate::combinators::stmt) fn column(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {
    /*
        COLUMN any_name
    */

    let (_, name) = seq!(Column, any_name)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_column() {
        test_parser!(
            source = "column foo",
            parser = column,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Column;
use pg_parser_core::scan;
