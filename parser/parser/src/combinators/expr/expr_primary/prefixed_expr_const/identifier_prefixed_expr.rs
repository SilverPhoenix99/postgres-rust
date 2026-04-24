pub(super) fn identifier_prefixed_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        column_ref (
              SCONST                                                => AexprConst
            | '(' func_arg_list ')' SCONST                          => AexprConst
            | '(' ( func_application_args )? ')' func_args_tail     => func_expr
            | ε                                                     => columnref
        )
    */

    let column_ref = column_ref(ctx)?;

    let name = match QualifiedName::try_from(column_ref) {
        Ok(name) => name,
        Err(column_ref) => {
            // columnref
            return Ok(column_ref.into())
        },
    };

    let Some(tail) = attr_tail(ctx).optional()? else {
        // columnref
        let mut name = name;
        let expr = match name.as_mut_slice() {
            [name] => ColumnRef::SingleName(mem::take(name)),
            _ => ColumnRef::Name(name)
        };
        return Ok(expr.into())
    };

    let expr = tailed_expr(name, tail);
    Ok(expr)
}

fn column_ref(ctx: &mut ParserContext) -> scan::Result<ColumnRef> {

    /*
          (IDENT | unreserved_keyword) ( indirection )?
        | col_name_keyword indirection
    */

    let (name, indirection) = alt!(
        seq!(
            alt!(
                identifier.map(Str::from),
                Unreserved.map(Str::from)
            ),
            located!(indirection).optional()
        ),
        seq!(
            ColumnName.map(Str::from),
            located!(indirection).map(Some)
        )
    ).parse(ctx)?;

    let column_ref = make_column_ref(name, indirection)?;
    Ok(column_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::ExprNode::StringConst;
    use pg_ast::FuncArgsKind;
    use pg_ast::FuncCall;
    use pg_ast::FuncCallExpr;
    use pg_ast::OverClause;
    use pg_ast::TypeName;
    use pg_ast::TypecastExpr;
    use test_case::test_matrix;

    #[test_matrix("foo" => Ok(
        ColumnRef::SingleName("foo".into()).into()
    ))]
    #[test_matrix("double" => Ok(
        ColumnRef::SingleName("double".into()).into()
    ))]
    #[test_matrix("foo.bar" => Ok( /* identifier */
        ColumnRef::Name(vec!["foo".into(), "bar".into()]).into()
    ))]
    #[test_matrix("double.baz" => Ok( /* Unreserved */
        ColumnRef::Name(vec!["double".into(), "baz".into()]).into()
    ))]
    #[test_matrix("between.qux" => Ok( /* ColumnName */
        ColumnRef::Name(vec!["between".into(), "qux".into()]).into()
    ))]
    #[test_matrix("foo.* '123'" => Ok(
        ColumnRef::WildcardName(vec!["foo".into()]).into()
    ))]
    #[test_matrix("double.* '123'" => Ok(
        ColumnRef::WildcardName(vec!["double".into()]).into()
    ))]
    #[test_matrix("between.* '123'" => Ok(
        ColumnRef::WildcardName(vec!["between".into()]).into()
    ))]
    #[test_matrix("foo.*()" => Ok(
        ColumnRef::WildcardName(vec!["foo".into()]).into()
    ))]
    #[test_matrix("double.*()" => Ok(
        ColumnRef::WildcardName(vec!["double".into()]).into()
    ))]
    #[test_matrix("between.*()" => Ok(
        ColumnRef::WildcardName(vec!["between".into()]).into()
    ))]
    #[test_matrix("foo '123'" => Ok(
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["foo".into()],
                type_modifiers: None,
            }
        ).into()
    ))]
    #[test_matrix("double '123'" => Ok(
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["double".into()],
                type_modifiers: None,
            }
        ).into()
    ))]
    #[test_matrix("foo.bar '123'" => Ok(
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["foo".into(), "bar".into()],
                type_modifiers: None,
            }
        ).into()
    ))]
    #[test_matrix("double.baz '123'" => Ok(
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["double".into(), "baz".into()],
                type_modifiers: None,
            }
        ).into()
    ))]
    #[test_matrix("between.qux '123'" => Ok(
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["between".into(), "qux".into()],
                type_modifiers: None,
            }
        ).into()
    ))]
    #[test_matrix("foo(1) '123'" => Ok(
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["foo".into()],
                type_modifiers: Some(vec![IntegerConst(1)]),
            }
        ).into()
    ))]
    #[test_matrix("double(1) '123'" => Ok(
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["double".into()],
                type_modifiers: Some(vec![IntegerConst(1)]),
            }
        ).into()
    ))]
    #[test_matrix("foo.bar(1) '123'" => Ok(
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["foo".into(), "bar".into()],
                type_modifiers: Some(vec![IntegerConst(1)]),
            }
        ).into()
    ))]
    #[test_matrix("double.baz(1) '123'" => Ok(
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["double".into(), "baz".into()],
                type_modifiers: Some(vec![IntegerConst(1)]),
            }
        ).into()
    ))]
    #[test_matrix("between.qux(1) '123'" => Ok(
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["between".into(), "qux".into()],
                type_modifiers: Some(vec![IntegerConst(1)]),
            }
        ).into()
    ))]
    #[test_matrix("foo() '123'" => Ok(
        FuncCallExpr::from(
            FuncCall::new(
                vec!["foo".into()],
                FuncArgsKind::Empty { order_within_group: None },
            )
        ).into()
    ))]
    #[test_matrix("double() '123'" => Ok(
        FuncCallExpr::from(
            FuncCall::new(
                vec!["double".into()],
                FuncArgsKind::Empty { order_within_group: None },
            )
        ).into()
    ))]
    #[test_matrix("foo.bar() over qux" => Ok(
        FuncCallExpr::from(
            FuncCall::new(
                vec!["foo".into(), "bar".into()],
                FuncArgsKind::Empty { order_within_group: None },
            )
        )
        .with_over(OverClause::WindowName("qux".into()))
        .into()
    ))]
    #[test_matrix("double.baz() filter (where 1)" => Ok(
        FuncCallExpr::from(
            FuncCall::new(
                vec!["double".into(), "baz".into()],
                FuncArgsKind::Empty { order_within_group: None },
            )
        )
        .with_agg_filter(IntegerConst(1))
        .into()
    ))]
    #[test_matrix("between.qux() filter (where 1)" => Ok(
        FuncCallExpr::from(
            FuncCall::new(
                vec!["between".into(), "qux".into()],
                FuncArgsKind::Empty { order_within_group: None },
            )
        )
        .with_agg_filter(IntegerConst(1))
        .into()
    ))]
    fn test_identifier_prefixed_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, identifier_prefixed_expr)
    }
}

use super::attr_tail;
use super::tailed_expr;
use crate::alt;
use crate::combinators::core::identifier;
use crate::combinators::core::Combinator;
use crate::combinators::expr::indirection;
use crate::combinators::make_column_ref;
use crate::located;
use crate::seq;
use crate::ParserContext;
use core::mem;
use pg_ast::ColumnRef;
use pg_ast::ExprNode;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_parser_core::scan;
use pg_parser_core::Optional;
