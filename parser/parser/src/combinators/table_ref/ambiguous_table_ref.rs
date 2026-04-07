pub(super) fn ambiguous_table_ref(ctx: &mut ParserContext) -> scan::Result<TableRef> {

    /*
          func_name func_application_args ( ordinality )? ( func_alias_clause )?
        | qualified_name ( '*' )? ( alias_clause )? ( tablesample_clause )?
    */

    let is_col_name = {
        let tok = ctx.stream_mut().peek();
        matches!(tok, Ok(Keyword(kw)) if kw.category() == ColumnName)
    };

    let Located(name, loc) = located!(any_name).parse(ctx)?;

    if matches!(ctx.stream_mut().peek(), Ok(Operator(OpenParenthesis))) {

        // it's func_name

        if is_col_name && name.len() == 1 {
            return Err(syntax(loc))
        }

        let (args, ordinality, alias) = seq!(
            func_application_args,
            ordinality.optional(),
            func_alias_clause.optional()
        ).parse(ctx)?;

        let func_call = FuncCall::new(name, args);
        let mut table_ref = FunctionTableRef::new(func_call);
        table_ref.set_ordinality(ordinality.is_some())
            .set_alias(alias);

        return Ok(table_ref.into())
    }

    // it's qualified_name

    let name = make_qualified_name(name, loc)?;

    let (_, alias, tablesample) = seq!(
        Mul.optional(),
        alias_clause.optional(),
        tablesample_clause.optional()
    ).parse(ctx)?;

    let relation = RelationExpr::new(name)
        .with_inherited(true);

    let mut table_ref = SampleTableRef::new(relation);
    table_ref.set_alias(alias)
        .set_table_sample(tablesample);

    Ok(table_ref.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::IntegerConst,
        pg_ast::FuncArgsKind::Empty,
        pg_ast::OneOrBoth::Left,
        pg_ast::RelationName,
        pg_ast::TableSample,
        pg_elog::parser::Error::Syntax,
        pg_elog::Error::Parser,
        pg_parser_core::scan::Error::ScanErr,
    };

    #[test_case("abort()" => Ok(
        FunctionTableRef::new(
            FuncCall::new(
                vec!["abort".into()],
                Empty { order_within_group: None }
            )
        )
        .into()
    ))]
    #[test_case("integer()" => matches Err(
        ScanErr(Located(Parser(Syntax), _))
    ))]
    #[test_case("integer.row() as a" => Ok(
        FunctionTableRef::new(
            FuncCall::new(
                vec!["integer".into(), "row".into()],
                Empty { order_within_group: None }
            )
        )
        .with_alias(Left("a".into()))
        .into()
    ))]
    #[test_case("foo() with ordinality" => Ok(
        FunctionTableRef::new(
            FuncCall::new(
                vec!["foo".into()],
                Empty { order_within_group: None }
            )
        )
        .with_ordinality(true)
        .into()
    ))]
    #[test_case("bar() with ordinality b" => Ok(
        FunctionTableRef::new(
            FuncCall::new(
                vec!["bar".into()],
                Empty { order_within_group: None }
            )
        )
        .with_ordinality(true)
        .with_alias(Left("b".into()))
        .into()
    ))]
    #[test_case("abort" => Ok(
        SampleTableRef::new(
            RelationExpr::new("abort")
                .with_inherited(true)
        )
        .into()
    ))]
    #[test_case("integer * tablesample fun1(1)" => Ok(
        SampleTableRef::new(
            RelationExpr::new("integer")
                .with_inherited(true)
        )
        .with_table_sample(
            TableSample::new(
                vec!["fun1".into()],
                vec![IntegerConst(1)],
            )
        )
        .into()
    ))]
    #[test_case("integer.row c" => Ok(
        SampleTableRef::new(
            RelationExpr::new(
                RelationName::new("row")
                    .with_schema("integer")
            )
            .with_inherited(true)
        )
        .with_alias("c")
        .into()
    ))]
    #[test_case("qux * as d tablesample fun2(2)" => Ok(
        SampleTableRef::new(
            RelationExpr::new("qux")
                .with_inherited(true)
        )
        .with_alias("d")
        .with_table_sample(
            TableSample::new(
                vec!["fun2".into()],
                vec![IntegerConst(2)],
            )
        )
        .into()
    ))]
    fn test_ambiguous_table_ref(source: &str) -> scan::Result<TableRef> {
        test_parser!(source, ambiguous_table_ref)
    }
}

use crate::combinators::any_name;
use crate::combinators::core::Combinator;
use crate::combinators::func_application_args;
use crate::combinators::make_qualified_name;
use crate::combinators::table_ref::alias_clause;
use crate::combinators::table_ref::func_alias_clause;
use crate::combinators::table_ref::ordinality;
use crate::combinators::table_ref::tablesample_clause;
use crate::context::ParserContext;
use crate::located;
use crate::seq;
use pg_ast::FuncCall;
use pg_ast::FunctionTableRef;
use pg_ast::RelationExpr;
use pg_ast::SampleTableRef;
use pg_ast::TableRef;
use pg_basics::Located;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::OperatorKind::Mul;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
use pg_parser_core::syntax;
