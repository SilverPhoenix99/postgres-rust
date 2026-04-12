pub(super) fn from_list(ctx: &mut ParserContext) -> scan::Result<Vec<TableRef>> {

    /*
        table_ref ( ',' table_ref )*
    */

    many!(sep = Comma, table_ref)
        .parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("foo, bar join qux using(baz)" => matches Ok(_))]
    fn test_from_list(source: &str) -> scan::Result<Vec<TableRef>> {
        test_parser!(source, from_list)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::table_ref;
use crate::context::ParserContext;
use crate::many;
use pg_ast::TableRef;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
