pub(in crate::combinators) fn select_stmt(ctx: &mut ParserContext) -> scan::Result<SelectStmt> {

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[ignore]
    #[test_case("select 1" => matches Ok(_))]
    fn test_select_stmt(source: &str) -> scan::Result<SelectStmt> {
        todo!()
    }
}

use crate::context::ParserContext;
use pg_ast::SelectStmt;
use pg_parser_core::scan;
