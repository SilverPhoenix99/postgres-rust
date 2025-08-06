pub(super) fn type_func_name_prefixed_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        type_func_name_keyword
        (
              SCONST                                            => AexprConst
            | '(' func_arg_list ')' SCONST                      => AexprConst
            | '(' ( func_application_args )? ')' func_args_tail => func_expr
        )
    */

    let (kw, tail) = seq!(TypeFuncName, attr_tail).parse(stream)?;
    let name = vec![Str::from(kw)];

    let expr = tailed_expr(name, tail);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::{IntegerConst, StringConst},
        FuncArgsKind,
        FuncCall,
        FuncCallExpr,
        StringTypecastExpr,
        TypeName,
    };
    use test_case::test_case;

    #[test_case("verbose 'foo'" => Ok(
        StringTypecastExpr::new(
            "foo",
            TypeName::Generic {
                name: vec![Str::from("verbose")],
                type_modifiers: None,
            }
        ).into()
    ))]
    #[test_case("current_schema(1) 'foo'" => Ok(
        StringTypecastExpr::new(
            "foo",
            TypeName::Generic {
                name: vec![Str::from("current_schema")],
                type_modifiers: Some(vec![IntegerConst(1)]),
            }
        ).into()
    ))]
    #[test_case("collation() filter (where 1)" => Ok(
        FuncCallExpr::from(
            FuncCall::new(
                vec![Str::from("collation")],
                FuncArgsKind::Empty { order_within_group: None }
            )
        )
        .with_agg_filter(IntegerConst(1))
        .into()
    ))]
    fn test_type_func_name_prefixed_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, type_func_name_prefixed_expr)
    }
}

use super::attr_tail;
use super::tailed_expr;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use pg_ast::ExprNode;
use pg_basics::Str;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
