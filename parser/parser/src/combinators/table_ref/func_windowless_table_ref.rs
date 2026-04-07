/// Alias: `func_table`
pub(super) fn func_windowless_table_ref(ctx: &mut ParserContext) -> scan::Result<FunctionTableRef> {

    /*
        func_expr_windowless ( ordinality )? ( func_alias_clause )?
    */

    let (function, ordinality, alias) = seq!(
                func_expr_windowless,
                ordinality.optional(),
                func_alias_clause.optional()
            ).parse(ctx)?;

    let mut table_ref = FunctionTableRef::new(function);
    table_ref.set_ordinality(ordinality.is_some());
    table_ref.set_alias(alias);
    Ok(table_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        FuncAliasColumn,
        FuncArgsKind,
        FuncCall,
        OneOrBoth::Both,
    };
    use test_case::test_case;

    #[test_case("foo()" => Ok(
        FunctionTableRef::new(FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::Empty { order_within_group: None }
        ))
    ))]
    #[test_case("bar() with ordinality" => Ok(
        FunctionTableRef::new(FuncCall::new(
            vec!["bar".into()],
            FuncArgsKind::Empty { order_within_group: None }
        ))
        .with_ordinality(true)
    ))]
    #[test_case("baz() as t(x)" => Ok(
        FunctionTableRef::new(FuncCall::new(
            vec!["baz".into()],
            FuncArgsKind::Empty { order_within_group: None }
        ))
        .with_alias(Both(
            "t".into(),
            vec![FuncAliasColumn::new("x")]
        ))
    ))]
    #[test_case("qux() with ordinality as s(y)" => Ok(
        FunctionTableRef::new(FuncCall::new(
            vec!["qux".into()],
            FuncArgsKind::Empty { order_within_group: None }
        ))
        .with_ordinality(true)
        .with_alias(Both(
            "s".into(),
            vec![FuncAliasColumn::new("y")]
        ))
    ))]
    fn test_func_windowless_table_ref(source: &str) -> scan::Result<FunctionTableRef> {
        test_parser!(source, func_windowless_table_ref)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::func_expr_windowless;
use crate::combinators::table_ref::func_alias_clause;
use crate::combinators::table_ref::ordinality;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::FunctionTableRef;
use pg_parser_core::scan;
