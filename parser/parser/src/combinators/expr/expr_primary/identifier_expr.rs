pub(super) fn identifier_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        IDENT
        (
            | ( indirection )? => columnref
            | ( attrs )?
            (
                | '(' ( func_application_args )? ')' within_group_clause filter_clause over_clause  => func_expr
                | '(' func_arg_list ( sort_clause )? ')' SCONST                                     => AexprConst
                | SCONST                                                                            => AexprConst
                | ε                                                                                 => columnref
            )
        )
    */

    let (name, indirection) = (
        identifier,
        located(indirection).optional()
    ).parse(stream)?;

    let column_ref = make_column_ref(name.into(), indirection)?;

    let attrs = match QualifiedName::try_from(column_ref) {
        Ok(attrs) => attrs,
        Err(column_ref) => {
            // ExprNode::ColumnRef
            return Ok(column_ref.into())
        },
    };

    let mut args = match attr_suffix(stream).optional()? {
        None => {
            // columnref
            let mut attrs = attrs;
            let expr = match attrs.as_mut_slice() {
                [name] => ColumnRef::SingleName(mem::take(name)),
                _ => ColumnRef::Name(attrs)
            };
            return Ok(expr.into())
        },
        Some(AttrSuffix::String(value)) => {
            // AexprConst
            let arg = ExprNode::StringConst(value);
            let type_name = TypeName::Generic { name: attrs, type_modifiers: None };
            let expr = TypecastExpr::new(arg, type_name);
            return Ok(expr.into())
        },
        Some(AttrSuffix::FuncArgs(args)) => args,
    };

    // PG-C matches for a string first, and then checks if function arguments are valid type modifiers.
    if let FuncArgsKind::All { args, order } = &mut args
        && let Some(value) = string(stream).optional()?
    {
        // C-PG won't allow the `ALL` keyword,
        // but it doesn't change the meaning of the expression,
        // so it's accepted here.

        let named_arg = args.iter()
            .find(|arg|
                matches!(arg, (FuncArgExpr::NamedValue { .. }, _))
            );

        if let Some((_, loc)) = named_arg {
            let err = InvalidNamedTypeModifier.at(loc.clone());
            return Err(err.into())
        }

        if let Some((_, loc)) = order {
            let err = InvalidOrderedTypeModifiers.at(loc.clone());
            return Err(err.into())
        }

        let type_modifiers = mem::take(args).into_iter()
            .map(|(arg, _)|
                if let FuncArgExpr::Unnamed(expr) = arg {
                    expr
                }
                else {
                    unreachable!("Already checked for named arguments above")
                }
            )
            .collect();

        // TODO: Check if it's a known type name, like "numeric", etc.
        let type_name = TypeName::Generic {
            name: attrs,
            type_modifiers: Some(type_modifiers)
        };

        // AexprConst
        let typecast = TypecastExpr::new(
            ExprNode::StringConst(value),
            type_name
        );

        return Ok(typecast.into())
    }

    let (group, filter, over) = (
        located(within_group_clause).optional(),
        filter_clause.optional(),
        over_clause.optional()
    ).parse(stream)?;

    if let Some((group, loc)) = group {
        args = match args {
            FuncArgsKind::Empty { .. } => FuncArgsKind::Empty { order_within_group: Some(group) },
            FuncArgsKind::Wildcard { .. } => FuncArgsKind::Wildcard { order_within_group: Some(group) },
            FuncArgsKind::All { args, order } => {

                if order.is_some() {
                    let err = MultipleOrderBy.at(loc);
                    return Err(err.into())
                }

                let order = FuncArgsOrder::WithinGroup(group);
                let order = Some((order, loc));

                FuncArgsKind::All { args, order }
            },
            FuncArgsKind::Distinct { order, .. } => {

                let err = if order.is_some() { MultipleOrderBy } else { DistinctWithinGroup };
                let err = err.at(loc);
                return Err(err.into())
            },
            FuncArgsKind::Variadic { order, .. } => {

                let err = if order.is_some() { MultipleOrderBy } else { VariadicWithinGroup };
                let err = err.at(loc);
                return Err(err.into())
            },
        }
    }

    let func_call = FuncCall::new(attrs, args, filter, over);
    Ok(func_call.into())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AttrSuffix {
    String(Box<str>),
    FuncArgs(FuncArgsKind),
}

fn attr_suffix(stream: &mut TokenStream) -> scan::Result<AttrSuffix> {

    /*
          SCONST
        | func_application_args
    */

    or((
        string.map(AttrSuffix::String),
        func_application_args.map(AttrSuffix::FuncArgs),
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::{IntegerConst, StringConst},
        pg_ast::OverClause,
        pg_ast::SortBy,
        pg_basics::Location,
    };

    #[test_case("foo",
        ColumnRef::SingleName("foo".into()).into()
    )]
    #[test_case("foo.bar",
        ColumnRef::Name(vec!["foo".into(), "bar".into()]).into()
    )]
    #[test_case("foo.*",
        ColumnRef::WildcardName(vec!["foo".into()]).into()
    )]
    #[test_case("foo.bar 'baz'",
        TypecastExpr::new(
            StringConst("baz".into()),
            TypeName::Generic {
                name: vec!["foo".into(), "bar".into()],
                type_modifiers: None
            }
        ).into()
    )]
    #[test_case("foo()",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::Empty { order_within_group: None },
            None,
            None
        ).into()
    )]
    #[test_case("foo() within group (order by 1)",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::Empty {
                order_within_group: Some(vec![
                    SortBy::new(IntegerConst(1), None, None)
                ])
            },
            None,
            None
        ).into()
    )]
    #[test_case("foo(*) within group (order by 1)",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::Wildcard {
                order_within_group: Some(vec![
                    SortBy::new(IntegerConst(1), None, None)
                ])
            },
            None,
            None
        ).into()
    )]
    #[test_case("foo(1) within group (order by 2) filter (where 3) over bar",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::All {
                args: vec![
                    (
                        FuncArgExpr::Unnamed(IntegerConst(1)),
                        Location::new(4..5, 1, 5)
                    )
                ],
                order: Some((
                    FuncArgsOrder::WithinGroup(vec![
                        SortBy::new(IntegerConst(2), None, None)
                    ]),
                    Location::new(7..13, 1, 8)
                ))
            },
            Some(IntegerConst(3)),
            Some(OverClause::WindowName("bar".into()))
        ).into()
    )]
    #[test_case("foo(1 order by 2) filter (where 3) over bar",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::All {
                args: vec![
                    (
                        FuncArgExpr::Unnamed(IntegerConst(1)),
                        Location::new(4..5, 1, 5)
                    )
                ],
                order: Some((
                    FuncArgsOrder::OrderBy(vec![
                        SortBy::new(IntegerConst(2), None, None)
                    ]),
                    Location::new(6..11, 1, 7)
                ))
            },
            Some(IntegerConst(3)),
            Some(OverClause::WindowName("bar".into()))
        ).into()
    )]
    #[test_case("foo(1)",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::All {
                args: vec![
                    (
                        FuncArgExpr::Unnamed(IntegerConst(1)),
                        Location::new(4..5, 1, 5)
                    )
                ],
                order: None
            },
            None,
            None
        ).into()
    )]
    #[test_case("foo(7) '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["foo".into()],
                type_modifiers: Some(vec![
                    IntegerConst(7)
                ])
            }
        ).into()
    )]
    fn test_identifier_expr(source: &str, expected: ExprNode) {
        test_parser!(source, identifier_expr, expected)
    }

    #[test_case("'some string'", AttrSuffix::String("some string".into()))]
    #[test_case("()", AttrSuffix::FuncArgs(
        FuncArgsKind::Empty { order_within_group: None }
    ))]
    fn test_attr_suffix(source: &str, expected: AttrSuffix) {
        test_parser!(source, attr_suffix, expected)
    }
}

use crate::combinators::expr::expr_primary::func_application::func_application_args;
use crate::combinators::expr::expr_primary::func_expr::filter_clause;
use crate::combinators::expr::expr_primary::func_expr::over_clause;
use crate::combinators::expr::expr_primary::func_expr::within_group_clause;
use crate::combinators::expr::indirection;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::located;
use crate::combinators::foundation::or;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::make_column_ref;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use core::mem;
use pg_ast::ColumnRef;
use pg_ast::ExprNode;
use pg_ast::FuncArgExpr;
use pg_ast::FuncArgsKind;
use pg_ast::FuncArgsOrder;
use pg_ast::FuncCall;
use pg_ast::TypeName;
use pg_ast::TypecastExpr;
use pg_basics::QualifiedName;
use pg_elog::parser::Error::DistinctWithinGroup;
use pg_elog::parser::Error::InvalidNamedTypeModifier;
use pg_elog::parser::Error::InvalidOrderedTypeModifiers;
use pg_elog::parser::Error::MultipleOrderBy;
use pg_elog::parser::Error::VariadicWithinGroup;
