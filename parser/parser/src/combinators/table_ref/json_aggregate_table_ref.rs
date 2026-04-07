pub(super) fn json_aggregate_table_ref(ctx: &mut ParserContext) -> scan::Result<FunctionTableRef> {

    /*
        json_aggregate_func ( ordinality )? ( func_alias_clause )?
    */

    let (func_expr, ordinality, alias) = seq!(
        json_aggregate_func,
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
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::{
            IntegerConst,
            StringConst,
        },
        pg_ast::JsonArrayAgg,
        pg_ast::JsonKeyValue,
        pg_ast::JsonObjectAgg,
        pg_ast::OneOrBoth,
    };

    #[test_case("json_arrayagg(1)" => Ok(
        FunctionTableRef::new(
            JsonArrayAgg::new(IntegerConst(1))
                .with_absent_on_null(true)
        )
    ))]
    #[test_case("json_arrayagg(2) with ordinality" => Ok(
        FunctionTableRef::new(
            JsonArrayAgg::new(IntegerConst(2))
                .with_absent_on_null(true)
        )
        .with_ordinality(true)
    ))]
    #[test_case("json_objectagg('foo': 3) as a" => Ok(
        FunctionTableRef::new(
            JsonObjectAgg::new(
                JsonKeyValue::new(
                    StringConst("foo".into()),
                    IntegerConst(3)
                )
            )
        )
        .with_alias(OneOrBoth::Left("a".into()))
    ))]
    #[test_case("json_objectagg('bar': 4) with ordinality as a" => Ok(
        FunctionTableRef::new(
            JsonObjectAgg::new(
                JsonKeyValue::new(
                    StringConst("bar".into()),
                    IntegerConst(4)
                )
            )
        )
        .with_ordinality(true)
        .with_alias(OneOrBoth::Left("a".into()))
    ))]
    fn json_aggregate_table_ref_parsing(source: &str) -> scan::Result<FunctionTableRef> {
        test_parser!(source, json_aggregate_table_ref)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::json_aggregate_func;
use crate::combinators::table_ref::func_alias_clause;
use crate::combinators::table_ref::ordinality;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::FunctionTableRef;
use pg_parser_core::scan;
