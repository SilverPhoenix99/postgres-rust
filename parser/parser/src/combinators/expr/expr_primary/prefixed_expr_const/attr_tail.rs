#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum AttrTail {
    Typecast {
        value: Box<str>,
        type_modifiers: Option<Vec<ExprNode>>,
    },
    FuncTail {
        args: FuncArgsKind,
        filter: Option<ExprNode>,
        over: Option<OverClause>,
    },
}

pub(super) fn attr_tail(stream: &mut TokenStream) -> scan::Result<AttrTail> {

    /*
          SCONST
        | '(' func_arg_list ')' SCONST
        | '(' ( func_application_args )? ')' func_args_tail
    */

    let mut args = match attr_suffix(stream)? {
        AttrSuffix::String(value) => {
            return Ok(AttrTail::Typecast {
                value,
                type_modifiers: None
            })
        }
        AttrSuffix::FuncArgs(args) => args,
    };

    // PG-C matches for a string first, and then checks if function arguments are valid type modifiers.
    if let FuncArgsKind::All { args, order } = &mut args
        && let Some(value) = string(stream).optional()?
    {
        // C-PG won't allow the `ALL` keyword,
        // but it doesn't change the meaning of the expression,
        // so it's accepted here.

        let type_modifiers = args.drain(..)
            .map(|Located(arg, loc)| {
                if arg.name().is_none() {
                    let (_, arg) = arg.into();
                    Ok(arg)
                }
                else {
                    Err(InvalidNamedTypeModifier.at_location(loc))
                }
            })
            .collect::<parser::LocatedResult<Vec<_>>>()?;

        if let Some(Located(_, loc)) = order {
            return Err(
                InvalidOrderedTypeModifiers
                    .at_location(loc.clone())
                    .into()
            )
        }

        // AexprConst
        return Ok(AttrTail::Typecast { value, type_modifiers: Some(type_modifiers) })
    }

    let tail = func_args_tail(stream)?;

    if let Some(Located(group, loc)) = tail.group {
        args = match args {
            FuncArgsKind::Empty { .. } => FuncArgsKind::Empty { order_within_group: Some(group) },
            FuncArgsKind::Wildcard { .. } => FuncArgsKind::Wildcard { order_within_group: Some(group) },
            FuncArgsKind::All { args, order } => {

                if order.is_some() {
                    return Err(MultipleOrderBy.at_location(loc).into())
                }

                let order = FuncArgsOrder::WithinGroup(group);
                let order = Some(Located(order, loc));

                FuncArgsKind::All { args, order }
            },
            FuncArgsKind::Distinct { order, .. } => {

                let err = if order.is_some() { MultipleOrderBy } else { DistinctWithinGroup };
                return Err(err.at_location(loc).into())
            },
            FuncArgsKind::Variadic { order, .. } => {

                let err = if order.is_some() { MultipleOrderBy } else { VariadicWithinGroup };
                return Err(err.at_location(loc).into())
            },
        }
    }

    let func_tail = AttrTail::FuncTail {
        args,
        filter: tail.filter,
        over: tail.over,
    };

    Ok(func_tail)
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

    alt!(
        string.map(AttrSuffix::String),
        func_application_args.map(AttrSuffix::FuncArgs),
    ).parse(stream)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FuncArgsTail {
    pub group: Option<Located<Vec<SortBy>>>,
    pub filter: Option<ExprNode>,
    pub over: Option<OverClause>,
}

fn func_args_tail(stream: &mut TokenStream) -> scan::Result<FuncArgsTail> {

    /*
        ( within_group_clause )? ( filter_clause )? ( over_clause )?
    */

    let (group, filter, over) = seq!(
        located!(within_group_clause).optional(),
        filter_clause.optional(),
        over_clause.optional()
    ).parse(stream)?;

    Ok(FuncArgsTail { group, filter, over })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_basics::Str;
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::IntegerConst,
        pg_ast::NamedValue,
        pg_basics::Location,
        pg_elog::Error::Parser,
        scan::Error::ScanErr,
    };

    #[test_case("'foo'" => Ok(
        AttrTail::Typecast {
            value: "foo".into(),
            type_modifiers: None
        }
    ))]
    #[test_case("() 'foo'" => Ok(
        AttrTail::FuncTail {
            args: FuncArgsKind::Empty { order_within_group: None },
            filter: None,
            over: None
        }
    ))]
    #[test_case("(1) 'foo'" => Ok(
        AttrTail::Typecast {
            value: "foo".into(),
            type_modifiers: Some(vec![IntegerConst(1)]),
        }
    ))]
    #[test_case("(1) over bar" => Ok(
        AttrTail::FuncTail {
            args: FuncArgsKind::All {
                args: vec![Located(
                    NamedValue::unnamed(IntegerConst(1)),
                    Location::new(1..2, 1, 2)
                )],
                order: None
            },
            filter: None,
            over: Some(OverClause::WindowName("bar".into()))
        }
    ))]
    #[test_case("(a := 1) 'foo'" => matches Err(ScanErr(
        Located(Parser(InvalidNamedTypeModifier), _)
    )))]
    #[test_case("(1 order by 2) 'foo'" => matches Err(ScanErr(
        Located(Parser(InvalidOrderedTypeModifiers), _)
    )))]
    #[test_case("(1 order by 2) within group (order by 3)" => matches Err(ScanErr(
        Located(Parser(MultipleOrderBy), _)
    )))]
    #[test_case("(distinct 1) within group (order by 3)" => matches Err(ScanErr(
        Located(Parser(DistinctWithinGroup), _)
    )))]
    #[test_case("(distinct 1 order by 2) within group (order by 3)" => matches Err(ScanErr(
        Located(Parser(MultipleOrderBy), _)
    )))]
    #[test_case("(variadic 1) within group (order by 3)" => matches Err(ScanErr(
        Located(Parser(VariadicWithinGroup), _)
    )))]
    #[test_case("(variadic 1 order by 2) within group (order by 3)" => matches Err(ScanErr(
        Located(Parser(MultipleOrderBy), _)
    )))]
    fn test_attr_tail(source: &str) -> scan::Result<AttrTail> {
        test_parser!(source, attr_tail)
    }

    #[test_case("'some string'" => Ok(AttrSuffix::String("some string".into())))]
    #[test_case("()" => Ok(AttrSuffix::FuncArgs(
        FuncArgsKind::Empty { order_within_group: None }
    )))]
    fn test_attr_suffix(source: &str) -> scan::Result<AttrSuffix> {
        test_parser!(source, attr_suffix)
    }

    #[test_case("", None, None, None)]
    #[test_case("within group (order by 1) filter (where 2) over foo",
        Some(Located(
            SortBy::new(
                IntegerConst(1),
                None,
                None
            ),
            Location::new(0..6, 1, 1)
        )),
        Some(IntegerConst(2)),
        Some("foo".into())
    )]
    fn test_func_args_tail(
        source: &str,
        group: Option<Located<SortBy>>,
        filter: Option<ExprNode>,
        over: Option<Str>
    ) {
        let expected = FuncArgsTail {
            group: group.map(|Located(sort, loc)| Located(vec![sort], loc)),
            filter,
            over: over.map(OverClause::WindowName),
        };

        test_parser!(source, func_args_tail, expected)
    }
}

use crate::combinators::expr::expr_primary::filter_clause;
use crate::combinators::expr::expr_primary::over_clause;
use crate::combinators::expr::expr_primary::within_group_clause;
use crate::combinators::func_application_args;
use pg_ast::ExprNode;
use pg_ast::FuncArgsKind;
use pg_ast::FuncArgsOrder;
use pg_ast::OverClause;
use pg_ast::SortBy;
use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_combinators::alt;
use pg_combinators::located;
use pg_combinators::seq;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_elog::parser;
use pg_elog::parser::Error::DistinctWithinGroup;
use pg_elog::parser::Error::InvalidNamedTypeModifier;
use pg_elog::parser::Error::InvalidOrderedTypeModifiers;
use pg_elog::parser::Error::MultipleOrderBy;
use pg_elog::parser::Error::VariadicWithinGroup;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::Optional;
