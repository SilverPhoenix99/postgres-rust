pub(super) fn select_table_ref(ctx: &mut ParserContext) -> scan::Result<SubselectTableRef> {

    /*
        '(' SelectStmt ')' ( alias_clause )?
    */

    let (subselect, alias) = seq!(
        paren!(select_stmt),
        alias_clause.optional()
    ).parse(ctx)?;

    let mut table_ref = SubselectTableRef::new(subselect);
    table_ref.set_alias(alias);

    Ok(table_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[ignore]
    #[test_case("select 1" => matches Ok(_))]
    fn test_select_table_ref(source: &str) -> scan::Result<SubselectTableRef> {
        todo!()
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::stmt::select_stmt;
use crate::combinators::table_ref::alias_clause;
use crate::context::ParserContext;
use crate::paren;
use crate::seq;
use pg_ast::SubselectTableRef;
use pg_parser_core::scan;
