pub(super) fn using_clause(ctx: &mut ParserContext) -> scan::Result<Vec<TableRef>> {

    /*
        USING from_list
    */

    let (_, table_refs) = seq!(Using, from_list)
        .parse(ctx)?;

    Ok(table_refs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("using foo, bar join qux using(baz)" => matches Ok(_))]
    fn test_using_clause(source: &str) -> scan::Result<Vec<TableRef>> {
        test_parser!(source, using_clause)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::stmt::from_list;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::TableRef;
use pg_lexer::Keyword::Using;
use pg_parser_core::scan;
