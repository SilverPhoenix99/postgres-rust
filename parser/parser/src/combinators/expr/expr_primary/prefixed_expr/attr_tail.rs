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
            .map(|(arg, loc)| match arg {
                FuncArgExpr::Unnamed(value) => Ok(value),
                _ => Err(InvalidNamedTypeModifier.at(loc)),
            })
            .collect::<parser::LocatedResult<Vec<_>>>()?;

        if let Some((_, loc)) = order {
            let err = InvalidOrderedTypeModifiers.at(loc.clone());
            return Err(err.into())
        }

        // AexprConst
        return Ok(AttrTail::Typecast { value, type_modifiers: Some(type_modifiers) })
    }

    let tail = func_args_tail(stream)?;

    if let Some((group, loc)) = tail.group {
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

    or((
        string.map(AttrSuffix::String),
        func_application_args.map(AttrSuffix::FuncArgs),
    )).parse(stream)
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

    let (group, filter, over) = (
        located(within_group_clause).optional(),
        filter_clause.optional(),
        over_clause.optional()
    ).parse(stream)?;

    Ok(FuncArgsTail { group, filter, over })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_basics::Str;
    use pg_elog::parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::IntegerConst,
        pg_basics::Location
    };

    #[test_case("'foo'",
        AttrTail::Typecast {
            value: "foo".into(),
            type_modifiers: None
        }
    )]
    #[test_case("() 'foo'",
        AttrTail::FuncTail {
            args: FuncArgsKind::Empty { order_within_group: None },
            filter: None,
            over: None
        }
    )]
    #[test_case("(1) 'foo'",
        AttrTail::Typecast {
            value: "foo".into(),
            type_modifiers: Some(vec![IntegerConst(1)]),
        }
    )]
    #[test_case("(1) over bar",
        AttrTail::FuncTail {
            args: FuncArgsKind::All {
                args: vec![(
                    FuncArgExpr::Unnamed(IntegerConst(1)),
                    Location::new(1..2, 1, 2)
                )],
                order: None
            },
            filter: None,
            over: Some(OverClause::WindowName("bar".into()))
        }
    )]
    fn test_attr_tail(source: &str, expected: AttrTail) {
        test_parser!(source, attr_tail, expected)
    }

    #[test_case("(a := 1) 'foo'",
        InvalidNamedTypeModifier.at(
            Location::new(1..2, 1, 2)
        )
    )]
    #[test_case("(1 order by 2) 'foo'",
        InvalidOrderedTypeModifiers.at(
            Location::new(3..8, 1, 4)
        )
    )]
    #[test_case("(1 order by 2) within group (order by 3)",
        MultipleOrderBy.at(
            Location::new(15..21, 1, 16)
        )
    )]
    #[test_case("(distinct 1) within group (order by 3)",
        DistinctWithinGroup.at(
            Location::new(13..19, 1, 14)
        )
    )]
    #[test_case("(distinct 1 order by 2) within group (order by 3)",
        MultipleOrderBy.at(
            Location::new(24..30, 1, 25)
        )
    )]
    #[test_case("(variadic 1) within group (order by 3)",
        VariadicWithinGroup.at(
            Location::new(13..19, 1, 14)
        )
    )]
    #[test_case("(variadic 1 order by 2) within group (order by 3)",
        MultipleOrderBy.at(
            Location::new(24..30, 1, 25)
        )
    )]
    fn test_attr_tail_err(source: &str, expected: parser::LocatedError) {
        test_parser!(source, attr_tail, Err(expected))
    }

    #[test_case("'some string'", AttrSuffix::String("some string".into()))]
    #[test_case("()", AttrSuffix::FuncArgs(
        FuncArgsKind::Empty { order_within_group: None }
    ))]
    fn test_attr_suffix(source: &str, expected: AttrSuffix) {
        test_parser!(source, attr_suffix, expected)
    }

    #[test_case("", None, None, None)]
    #[test_case("within group (order by 1) filter (where 2) over foo",
        Some((
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
            group: group.map(|(sort, loc)| (vec![sort], loc)),
            filter,
            over: over.map(OverClause::WindowName),
        };

        test_parser!(source, func_args_tail, expected)
    }
}

use crate::combinators::expr::expr_primary::filter_clause;
use crate::combinators::expr::expr_primary::func_application::func_application_args;
use crate::combinators::expr::expr_primary::over_clause;
use crate::combinators::expr::expr_primary::within_group_clause;
use crate::combinators::foundation::located;
use crate::combinators::foundation::or;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_ast::FuncArgExpr;
use pg_ast::FuncArgsKind;
use pg_ast::FuncArgsOrder;
use pg_ast::OverClause;
use pg_ast::SortBy;
use pg_basics::Located;
use pg_elog::parser;
use pg_elog::parser::Error::DistinctWithinGroup;
use pg_elog::parser::Error::InvalidNamedTypeModifier;
use pg_elog::parser::Error::InvalidOrderedTypeModifiers;
use pg_elog::parser::Error::MultipleOrderBy;
use pg_elog::parser::Error::VariadicWithinGroup;
