pub(super) fn func_subexpr_table_ref(ctx: &mut ParserContext) -> scan::Result<FunctionTableRef> {

    /*
        func_expr_common_subexpr ( ordinality )? ( func_alias_clause )?
    */

    let (func_expr, ordinality, alias) = seq!(
        func_expr_common_subexpr,
        ordinality.optional(),
        func_alias_clause.optional()
    ).parse(ctx)?;

    let mut table_ref = FunctionTableRef::new(func_expr);
    table_ref.set_ordinality(ordinality.is_some())
        .set_alias(alias);

    Ok(table_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        OneOrBoth,
        SqlFunction,
    };
    use test_case::test_case;

    #[test_case("current_catalog" => Ok(
        FunctionTableRef::new(SqlFunction::CurrentCatalog)
    ))]
    #[test_case("current_date with ordinality" => Ok(
        FunctionTableRef::new(SqlFunction::CurrentDate)
            .with_ordinality(true)
    ))]
    #[test_case("current_role as foo" => Ok(
        FunctionTableRef::new(SqlFunction::CurrentRole)
            .with_alias(OneOrBoth::Left("foo".into()))
    ))]
    #[test_case("current_schema with ordinality as bar" => Ok(
        FunctionTableRef::new(SqlFunction::CurrentSchema)
            .with_ordinality(true)
            .with_alias(OneOrBoth::Left("bar".into()))
    ))]
    fn test_func_subexpr_table_ref(source: &str) -> scan::Result<FunctionTableRef> {
        test_parser!(source, func_subexpr_table_ref)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::expr::func_expr_common_subexpr;
use crate::combinators::table_ref::func_alias_clause;
use crate::combinators::table_ref::ordinality;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::FunctionTableRef;
use pg_parser_core::scan;
