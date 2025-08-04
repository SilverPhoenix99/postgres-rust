pub fn identifier_prefixed_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        column_ref (
              SCONST                                                => AexprConst
            | '(' func_arg_list ')' SCONST                          => AexprConst
            | '(' ( func_application_args )? ')' func_args_tail     => func_expr
            | ε                                                     => columnref
        )
    */

    let column_ref = column_ref(stream)?;

    let name = match QualifiedName::try_from(column_ref) {
        Ok(name) => name,
        Err(column_ref) => {
            // columnref
            return Ok(column_ref.into())
        },
    };

    let Some(tail) = attr_tail(stream).optional()? else {
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

fn column_ref(stream: &mut TokenStream) -> scan::Result<ColumnRef> {

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
    ).parse(stream)?;

    let column_ref = make_column_ref(name, indirection)?;
    Ok(column_ref)
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
        OverClause,
        TypeName,
        TypecastExpr,
    };
    use test_case::test_case;

    #[test_case("foo",
        ColumnRef::SingleName("foo".into()).into()
    )]
    #[test_case("double",
        ColumnRef::SingleName("double".into()).into()
    )]
    #[test_case("foo.bar", // identifier
        ColumnRef::Name(vec!["foo".into(), "bar".into()]).into()
    )]
    #[test_case("double.baz", // Unreserved
        ColumnRef::Name(vec!["double".into(), "baz".into()]).into()
    )]
    #[test_case("between.qux", // ColumnName
        ColumnRef::Name(vec!["between".into(), "qux".into()]).into()
    )]
    #[test_case("foo.* '123'",
        ColumnRef::WildcardName(vec!["foo".into()]).into()
    )]
    #[test_case("double.* '123'",
        ColumnRef::WildcardName(vec!["double".into()]).into()
    )]
    #[test_case("between.* '123'",
        ColumnRef::WildcardName(vec!["between".into()]).into()
    )]
    #[test_case("foo.*()",
        ColumnRef::WildcardName(vec!["foo".into()]).into()
    )]
    #[test_case("double.*()",
        ColumnRef::WildcardName(vec!["double".into()]).into()
    )]
    #[test_case("between.*()",
        ColumnRef::WildcardName(vec!["between".into()]).into()
    )]
    #[test_case("foo '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["foo".into()],
                type_modifiers: None,
            }
        ).into()
    )]
    #[test_case("double '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["double".into()],
                type_modifiers: None,
            }
        ).into()
    )]
    #[test_case("foo.bar '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["foo".into(), "bar".into()],
                type_modifiers: None,
            }
        ).into()
    )]
    #[test_case("double.baz '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["double".into(), "baz".into()],
                type_modifiers: None,
            }
        ).into()
    )]
    #[test_case("between.qux '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["between".into(), "qux".into()],
                type_modifiers: None,
            }
        ).into()
    )]
    #[test_case("foo(1) '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["foo".into()],
                type_modifiers: Some(vec![IntegerConst(1)]),
            }
        ).into()
    )]
    #[test_case("double(1) '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["double".into()],
                type_modifiers: Some(vec![IntegerConst(1)]),
            }
        ).into()
    )]
    #[test_case("foo.bar(1) '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["foo".into(), "bar".into()],
                type_modifiers: Some(vec![IntegerConst(1)]),
            }
        ).into()
    )]
    #[test_case("double.baz(1) '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["double".into(), "baz".into()],
                type_modifiers: Some(vec![IntegerConst(1)]),
            }
        ).into()
    )]
    #[test_case("between.qux(1) '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["between".into(), "qux".into()],
                type_modifiers: Some(vec![IntegerConst(1)]),
            }
        ).into()
    )]
    #[test_case("foo() '123'",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::Empty { order_within_group: None },
            None,
            None
        ).into()
    )]
    #[test_case("double() '123'",
        FuncCall::new(
            vec!["double".into()],
            FuncArgsKind::Empty { order_within_group: None },
            None,
            None
        ).into()
    )]
    #[test_case("foo.bar() over qux",
        FuncCall::new(
            vec!["foo".into(), "bar".into()],
            FuncArgsKind::Empty { order_within_group: None },
            None,
            Some(OverClause::WindowName("qux".into()))
        ).into()
    )]
    #[test_case("double.baz() filter (where 1)",
        FuncCall::new(
            vec!["double".into(), "baz".into()],
            FuncArgsKind::Empty { order_within_group: None },
            Some(IntegerConst(1)),
            None
        ).into()
    )]
    #[test_case("between.qux() filter (where 1)",
        FuncCall::new(
            vec!["between".into(), "qux".into()],
            FuncArgsKind::Empty { order_within_group: None },
            Some(IntegerConst(1)),
            None
        ).into()
    )]
    fn test_identifier_prefixed_expr(source: &str, expected: ExprNode) {
        test_parser!(source, identifier_prefixed_expr, expected)
    }
}

use super::attr_tail;
use super::tailed_expr;
use crate::combinators::expr::indirection;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::located;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::make_column_ref;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use core::mem;
use pg_ast::ColumnRef;
use pg_ast::ExprNode;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::Unreserved;
